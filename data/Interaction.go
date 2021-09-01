package data

import (
	"github.com/gofiber/websocket/v2"
)

type InteractionApplication struct {
	ApplicationID string `json:"application_id"`
}

type TypeConnection struct {
	PublicKey string `json:"public_key"`
	BotID     string `json:"bot_id"`
	BotName   string `json:"bot_name"`
	Date      int    `json:"date"`
}

type TypeConnectionSaved struct {
	PublicKey string `json:"public_key"`
	BotID     string `json:"bot_id"`
	BotName   string `json:"bot_name"`
	Date      int    `json:"date"`
	Session   websocket.Conn
}
