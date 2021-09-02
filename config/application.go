package config

import (
	"fmt"
	"io/ioutil"

	"gopkg.in/yaml.v2"
)

type Configuration struct {
	Interaction struct {
		PublicKey    []string `yaml:"publicKey"`
		SlashCommand struct {
			RespondingInteractionLate bool `yaml:"responding_interaction_late"`
		} `yaml:"slash_command"`
	} `yaml:"interaction"`
	Server struct {
		Port            int  `yaml:"port"`
		Debug           bool `yaml:"server_debug"`
		DebugHard       bool `yaml:"server_debug_hard"`
		LogConnectionWs bool `yaml:"log_connection_ws"`
	} `yaml:"server"`
	WS struct {
		Local  bool   `yaml:"local"`
		Secret string `yaml:"secret"`
	} `yaml:"ws"`
}

func Load() *Configuration {
	configuration := &Configuration{}
	file, err := ioutil.ReadFile("configuration.yaml")
	if err != nil {
		file, err = ioutil.ReadFile("configuration.yml")
		if err != nil {
			fmt.Printf("Error ~> YAML file: %s\n", err)
		}
	}

	yaml.Unmarshal(file, configuration)
	return configuration
}
