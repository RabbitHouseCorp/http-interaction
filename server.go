package main

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"regexp"
	color "github.com/fatih/color"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/websocket/v2"
	Configuration "github.com/rabbithousecorp/post-interaction/config"
	AccountsService "github.com/rabbithousecorp/post-interaction/data"
)

type CloseConnection struct {
	Message string `json:"message_ws"`
	Type    int    `json:"type"`
}

var bots = make(map[string]AccountsService.TypeConnectionSaved)

func main() {
	color.Cyan("Post Interaction - V1")
	color.Yellow("If you find an error in the functions, issue an issue on Github! Thanks.")
	configuration := Configuration.Load()
	log.Println("starting the web server to receive interactions!")
	server := fiber.New(fiber.Config{
		AppName:               "Post Interaction",
		DisableStartupMessage: true,
	})

	server.Use("/ws", func(c *fiber.Ctx) error {
		if websocket.IsWebSocketUpgrade(c) {
			c.Locals("allowed", true)
			if c.Get("Authorization", "") == "" {
				return c.Context().Conn().Close()
			} else {
				if c.Get("Authorization", "") == base64.RawStdEncoding.EncodeToString([]byte(configuration.WS.Secret)) {
					return c.Next()
				} else {
					return c.Context().Conn().Close()
				}
			}

		}
		return c.Context().Conn().Close()
	})

	server.Get("/ws", websocket.New(func(c *websocket.Conn) {
		if configuration.WS.Local == true {
			if !(regexp.MustCompile(`\:.(.*)`).ReplaceAllString(c.Conn.LocalAddr().String(), "") == "127.0.0.1") && !(regexp.MustCompile(`\:.(.*)`).ReplaceAllString(c.Conn.LocalAddr().String(), "") == "localhost") {
				c.Close()
		
				return
			}
		}
		var (
			mt  int
			msg []byte
			err error
		)

		for {
			if mt, msg, err = c.ReadMessage(); err != nil {
				log.Println("read:", err)
				DeleteConnectionSocket(c)
				break
			}
			var data AccountsService.TypeConnection
			err := json.Unmarshal(msg, &data)
			if err != nil {
				fmt.Println(err)
				c.Close()
				break
			}
			if data.BotID == "" {
				c.Close()
				break
			}
			if data.PublicKey == "" {
				c.Close()
				break
			}
			if data.BotName == "" {
				c.Close()
				break
			}
			if data.Date == 0 {
				c.Close()
				break
			}
			if mt == -1 {
				return
			}

			botConnection := bots[data.BotID]
			
			if botConnection.Session != nil {
				UpdateConnection(botConnection, c)
			} else {
				Add(data, c)
			}

			c.WriteJSON(data)
			if configuration.Server.LogConnectionWs {
				if err = c.WriteMessage(mt, msg); err != nil {
					log.Println("write:", err)
					break
				}
			}

		}
	}))
	server.Use(func(c *fiber.Ctx) error {
		if string(c.Context().Path()) != "/interaction" {
			return c.Context().Conn().Close() // Refuse this random endpoint.
		} else {
			if string(c.Context().Method()) == "POST" {
				return c.Next() // OK!
			} else {
				return c.Context().Conn().Close() // Refuse this random endpoint.
			}
		}
	})
	a, _ := SizeItemString(configuration.Interaction.PublicKey)
	server.Post("/interaction", func(c *fiber.Ctx) error {
		if a == 0 {
			// There is no public key! Check configuration.yaml pls
			c.Status(500)
			return nil
		}
		// You can follow through the documentary how this encryption basically works.
		signature := c.Get("X-Signature-Ed25519", "")   // Get this header
		timestamp := c.Get("X-Signature-Timestamp", "") // Get this header
		if signature == "" {
			return c.Context().Conn().Close()
		}
		if timestamp == "" {
			return c.Context().Conn().Close()
		}
		for _, b := range configuration.Interaction.PublicKey {
			value, _ := hex.DecodeString(b) // Public key that you get from the Discord app portal

			data := []byte(timestamp + string(c.Body()))   // Buffer
			signatureHex, _ := hex.DecodeString(signature) // Transform Signature to Hex
			pKey := ed25519.PublicKey(value)
			if pKey == nil {
				fmt.Println("Please! Check the public key in the configuration.")
				fmt.Println("Remembering that it cannot contain space!")
				c.Status(403)
				return nil
			}
			verifySignature := ed25519.Verify(ed25519.PublicKey(value), data, signatureHex) // Let's check the encryption.

			if verifySignature {
				// Hmm... that looks good. let's return the signal
				var checkApplication AccountsService.InteractionApplication

				err := json.Unmarshal(c.Body(), &checkApplication)
				if err != nil {
					return c.Status(200).JSON(&fiber.Map{
						"type": 1,
					})
				}
				getSession := bots[checkApplication.ApplicationID]

				if getSession.BotID != "" {
					if getSession.PublicKey != b {
						if getSession.BotID == checkApplication.ApplicationID {
							if c.Body() != nil {
								c.Body()
								if getSession.Session != nil {
									getSession.Session.WriteMessage(websocket.TextMessage, []byte("129"+ string(c.Body())))
								}
							}
						}
					}
				}

				// Slash Command
				if configuration.Interaction.SlashCommand.RespondingInteractionLate {
					if checkApplication.Type == 2 {
						return c.Status(200).JSON(&fiber.Map{
							"type": 5,
						})
					}
				}

				return c.Status(200).JSON(&fiber.Map{
					"type": 1,
				})
			}
		}
		// Wow! There's something wrong with this encryption!
		return c.Status(401).Send([]byte(`Invalid Request Signature`))
	})

	server.Listen(":2000")
}

func DeleteConnectionSocket(c *websocket.Conn) {
	for a, b := range bots {
		if c.Conn.LocalAddr() == b.Session.LocalAddr() {
			delete(bots, a)
		}
	}
}
func UpdateConnection(Bot AccountsService.TypeConnectionSaved, c *websocket.Conn) {
	bots[Bot.BotID] = AccountsService.TypeConnectionSaved{
		BotID:   Bot.BotID,
		BotName: Bot.BotName,
		Date:    Bot.Date,

		Session: c,
	}
}

func Delete(Bot AccountsService.TypeConnectionSaved) {
}

func DeleteByID(id string) {
}

func IsValid(c *websocket.Conn) bool {
	for _, connection := range bots {
		if connection.Session.LocalAddr() == c.LocalAddr() {
			return true
		}
	}
	return false
}

func IsValidPublicKeyMap(key string) bool {
	for _, connection := range bots {
		if connection.PublicKey == key {
			return true
		}
	}
	return false
}

func Add(Bot AccountsService.TypeConnection, c *websocket.Conn) AccountsService.TypeConnectionSaved {
	bots[Bot.BotID] = AccountsService.TypeConnectionSaved{
		BotID:   Bot.BotID,
		BotName: Bot.BotName,
		Date:    Bot.Date,

		Session: c,
	}

	return bots[Bot.BotID]
}

func SizeItemString(items []string) (int, string) {
	nb := 0
	lastItem := ""
	for _, a := range items {
		lastItem = a
		nb++
	}
	return nb, lastItem
}
