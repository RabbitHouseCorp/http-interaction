// Version 
package main

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"log"
	"regexp"
	"sync"

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

var (
	interactionInterface       = make(map[string]AccountsService.IPacket)
	tokenInteractionMap        = make(map[string]AccountsService.TokenInteractionData)
	tokenInteractionSync       = make(map[string]AccountsService.TokenInteractionDataSync)
	tokenInteractionSyncQueued = make(map[string]string)
)

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

			if GetBot(c) == false {

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
			} else {

				var data AccountsService.IPacket
				err := json.Unmarshal(msg, &data)
				if err != nil {
					fmt.Println(err)
					break
				}
				if data.Type == 3 {
					AddInteraction(AccountsService.IPacket{
						ID:       data.ID,
						Type:     data.Type,
						Metadata: data.Metadata,
					})
				}

				if data.Type == 89 {
					c.WriteJSON(&fiber.Map{
						"type_ws": 1001,
						"ping":    true,
					})
				}

				if data.Type == 90 {
					c.WriteJSON(&fiber.Map{
						"type_ws": 1002,
						"pong":    true,
					})
				}
				if data.Type == 95 {
					await := tokenInteractionSync[data.TokenInteraction].Wg
					AddToken(AccountsService.TokenInteractionData{
						TokenInteraction: data.TokenInteraction,
						Content:          data.Content,
						PingPong:         data.PingPong,
					})
					if tokenInteractionSyncQueued[data.TokenInteraction] == "" {
						if await != nil {
							tokenInteractionSyncQueued[data.TokenInteraction] = "true"
							await.Done()
						}
					}

				}

			}

		}
	}))

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
			c.Status(403)
			return nil
		}
		if timestamp == "" {
			c.Status(403)
			return nil
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

				if !(checkApplication.Type == 1) {
					if getSession.BotID == "" {
						return c.Status(200).JSON(&fiber.Map{
							"type": 4,
							"data": &fiber.Map{
								"tts":     false,
								"content": "There was a problem with the interaction!",
								"embeds": []fiber.Map{
									0: {
										"color":       "#ff1212",
										"description": "Failed to communicate with an interaction.",
									},
								},
								"allowed_mentions": &fiber.Map{
									"parse": []fiber.Map{},
								},
							},
						})
					}
				}

				// Slash Command

				if checkApplication.Type == 2 {
					MessageBucket([]byte("129"+string(c.Body())), *getSession.Session)

					return c.Status(200).JSON(&fiber.Map{
						"type": 5,
					})
				}
				if checkApplication.Type == 3 {
					var await sync.WaitGroup
					if tokenInteractionSync[checkApplication.Token].TokenInteraction == "" {
						AddTokenSync(AccountsService.TokenInteractionDataSync{
							TokenInteraction: checkApplication.Token,
							Wg:               &await,
						})
						await.Add(1)
						MessageBucket([]byte("129"+string(c.Body())), *getSession.Session)
						await.Wait()
					} else {
						MessageBucket([]byte("129"+string(c.Body())), *getSession.Session)
					}

					if !CheckToken(checkApplication.Token) {
						return c.Status(200).JSON(tokenInteractionMap[checkApplication.Token].Content)
					} else {
						return c.Status(200).JSON(&fiber.Map{
							"type": 1,
						})
					}
				}
				return c.Status(200).JSON(&fiber.Map{
					"type": 1,
				})
			}
		}
		// Wow! There's something wrong with this encryption!
		fmt.Println("Invalid Request Signature")
		return c.Status(401).Send([]byte(`Invalid Request Signature`))
	})

	log.Fatal(server.Listen(":2000"))
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

func UpdateConnectionSocket(c *websocket.Conn) {
	for a, b := range bots {
		if c.Conn.LocalAddr() == b.Session.LocalAddr() {
			bot := bots[a]
			bot.Session = c
			bots[a] = bot
		}
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

func MessageBucket(message []byte, ws websocket.Conn) {
	ws.WriteMessage(websocket.BinaryMessage, message)
}

func GetBot(c *websocket.Conn) bool {
	for _, b := range bots {
		if c.Conn.LocalAddr() == b.Session.LocalAddr() {
			return true
		}
	}
	return false
}

func AddToken(InteractionData AccountsService.TokenInteractionData) {
	tokenInteractionMap[InteractionData.TokenInteraction] = InteractionData
}

func AddTokenSync(InteractionData AccountsService.TokenInteractionDataSync) {
	tokenInteractionSync[InteractionData.TokenInteraction] = InteractionData
}

func CheckToken(Token string) bool {
	if tokenInteractionMap[Token].TokenInteraction != "" {
		return false
	} else {
		return true
	}
}

func AddInteraction(Interaction AccountsService.IPacket) {
	interactionInterface[Interaction.ID] = AccountsService.IPacket{
		ID:       Interaction.ID,
		Type:     Interaction.Type,
		Metadata: Interaction.Metadata,
	}
}

func GetInteraction(id string) *AccountsService.IPacket {
	for _, b := range interactionInterface {
		if id != b.ID {
			return &b
		}
	}
	return nil
}

func RemoveInteraction(Interaction AccountsService.IPacket) {
	delete(interactionInterface, Interaction.ID)
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
