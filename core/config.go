package core

import (
	"errors"
	"fmt"
	"os"
	"strings"

	"github.com/spf13/viper"
)

type Config struct {
	LogsDir string
}

func expandHome(path string) (string, error) {
	path = os.ExpandEnv(path) // Handles $HOME
	if strings.HasPrefix(path, "~/") {
		home, err := os.UserHomeDir()
		if err != nil {
			return "", fmt.Errorf("Unable to find user home directory")
		}
		path = strings.Replace(path, "~/", home+"/", 1)
	}

	return path, nil
}

func InitConfig() (Config, error) {
	viper.SetConfigName("config")
	viper.SetConfigType("toml")
	viper.AddConfigPath("$HOME/.config/grimoire/")

	var config = Config{}

	// Ensure config exists and is well-formed
	if err := viper.ReadInConfig(); err != nil {
		if _, ok := errors.AsType[viper.ConfigFileNotFoundError](err); ok {
			return Config{}, fmt.Errorf("%w. Please run `init` to create it.", err)
		}
		if _, ok := errors.AsType[viper.ConfigParseError](err); ok {
			return Config{}, fmt.Errorf("%w. Please run `init` to re-init.", err)
		}
	}

	// Check critical keys
	if ok := viper.IsSet("logs_dir"); !ok {
		return Config{}, fmt.Errorf("`logs_dir` not set in config. Please set to your logs directory.")
	}

	// Set keys
	config.LogsDir = viper.GetString("logs_dir")

	// Resolve logsDir
	var err error
	config.LogsDir, err = expandHome(config.LogsDir)
	if err != nil {
		return Config{}, err
	}

	return config, nil
}
