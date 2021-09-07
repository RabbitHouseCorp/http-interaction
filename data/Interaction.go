package data

import (
	"github.com/gofiber/websocket/v2"
)

type InteractionApplication struct {
	ApplicationID string `json:"application_id"`
	Type          int    `json:"type"`
	ID            string `json:"id"`
}

type TypeConnection struct {
	PublicKey string `json:"public_key"`
	BotID     string `json:"bot_id"`
	BotName   string `json:"bot_name"`
	Date      int    `json:"date"`
}

type TypeConnectionSaved struct {
	PublicKey string          `json:"public_key"`
	BotID     string          `json:"bot_id"`
	BotName   string          `json:"bot_name"`
	Date      int             `json:"date"`
	Session   *websocket.Conn `json:"session"`
}

type IPacket struct {
	ID       string      `json:"id"`
	Type     int         `json:"type"`
	Metadata interface{} `json:"data"`
}
