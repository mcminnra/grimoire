package config

import (
	"errors"
	"fmt"
	"io/fs"
	"os"
	"path/filepath"
	"strings"

	"github.com/BurntSushi/toml"
)

const configLocation = ".config/grimoire/config"

type Config struct {
	LogsDir string `toml:"logs_dir"`
}

type ConfigError struct {
	Op  string // "system" | "read" | "parse" | "write", system means something about the underlying OS prohibits the config
	Err error
}

func (e *ConfigError) Error() string {
	return fmt.Sprintf("%v", e.Err)
}
func (e *ConfigError) Unwrap() error { return e.Err }

// expandHome expands a path that contains $HOME or starts with ~/
func expandHome(path string) (string, error) {
	path = os.ExpandEnv(path) // Handles $HOME
	if strings.HasPrefix(path, "~/") {
		home, err := os.UserHomeDir()
		if err != nil {
			return "", err
		}
		path = strings.Replace(path, "~/", home+"/", 1)
	}

	return path, nil
}

// NormalizeConfig normalizes values in config to a more usable state
func NormalizeConfig(config Config) (Config, error) {
	// Resolve LogsDir
	var err error
	config.LogsDir, err = expandHome(config.LogsDir)
	if err != nil {
		return Config{}, &ConfigError{Op: "parse", Err: fmt.Errorf("Unable to normalize logs_dir")}
	}

	return config, nil
}

// validateDir validates a dir actually exists on the filesystem
func validateDir(path string) error {
	info, err := os.Stat(path)
	if err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return fmt.Errorf("%q does not exist", path)
		}
		return err // permission denied, etc.
	}
	if !info.IsDir() {
		return fmt.Errorf("%q exists but is not a directory", path)
	}

	return nil
}

// ValidateConfig checks the config for errors
func ValidateConfig(config Config) error {
	if err := validateDir(config.LogsDir); err != nil {
		return &ConfigError{Op: "parse", Err: fmt.Errorf("Unable to validate logs_dir: %w", err)}
	}

	return nil
}

// Path locates the correct home config location for the user
func Path() (string, error) {
	home, err := os.UserHomeDir()
	if err != nil {
		return "", err
	}
	fullPath := filepath.Join(home, configLocation)

	return fullPath, nil
}

// Exists reports whether a config file already exists on disk
func Exists() (bool, error) {
	configPath, err := Path()
	if err != nil {
		return false, &ConfigError{Op: "system", Err: fmt.Errorf("Unable to form config location")}
	}
	if _, err := os.Stat(configPath); err != nil {
		if errors.Is(err, fs.ErrNotExist) {
			return false, nil
		}
		return false, &ConfigError{Op: "system", Err: err}
	}
	return true, nil
}

// GetConfig reads config path on disk and returns a Config
func GetConfig() (Config, error) {
	var config = Config{}

	// Get config path
	configPath, err := Path()
	if err != nil {
		return Config{}, &ConfigError{Op: "system", Err: fmt.Errorf("Unable to form config location %s", filepath.Dir(configPath))}
	}

	// Read and parse
	b, err := os.ReadFile(configPath)
	if err != nil {
		return Config{}, &ConfigError{Op: "read", Err: fmt.Errorf("Unable to read %s. Does it exist?", configPath)}
	}
	err = toml.Unmarshal(b, &config)
	if err != nil {
		return Config{}, &ConfigError{Op: "parse", Err: fmt.Errorf("Unable to parse %s - %w", configPath, err)}
	}

	// Normalize
	config, err = NormalizeConfig(config)
	if err != nil {
		return Config{}, &ConfigError{Op: "parse", Err: err}
	}

	// Validate
	if err := ValidateConfig(config); err != nil {
		return Config{}, &ConfigError{Op: "parse", Err: err}
	}

	return config, nil
}

// WriteConfig persists config to disk and returns path written to
func WriteConfig(config Config) (string, error) {
	// Get config path
	configPath, err := Path()
	if err != nil {
		return "", &ConfigError{Op: "system", Err: fmt.Errorf("Unable to form config location")}
	}

	// Normalize
	config, err = NormalizeConfig(config)
	if err != nil {
		return "", &ConfigError{Op: "parse", Err: err}
	}

	// Validate
	if err := ValidateConfig(config); err != nil {
		return "", &ConfigError{Op: "parse", Err: err}
	}

	// Form config dir
	if err := os.MkdirAll(filepath.Dir(configPath), 0o755); err != nil {
		return "", &ConfigError{Op: "write", Err: fmt.Errorf("Unable to create config directory %s", filepath.Dir(configPath))}
	}

	// Marshall and write config
	b, err := toml.Marshal(config)
	if err != nil {
		return "", &ConfigError{Op: "parse", Err: fmt.Errorf("Unable to marshall config - %w", err)}
	}
	if err := os.WriteFile(configPath, b, 0o644); err != nil {
		return "", &ConfigError{Op: "write", Err: fmt.Errorf("Unable to write config %s", configPath)}
	}

	return configPath, nil
}
