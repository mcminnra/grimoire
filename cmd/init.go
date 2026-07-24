package cmd

import (
	"fmt"
	"grimoire/core/config"

	"github.com/charmbracelet/huh"
	"github.com/spf13/cobra"
)

var (
	initLogsDir string
	initForce   bool
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Init Grimoire config",
	Long:  "Init Grimoire config",
	RunE: func(cmd *cobra.Command, args []string) error {
		// Check for an existing config before prompting so the user isn't refused after filling the form
		exists, err := config.Exists()
		if err != nil {
			return err
		}
		if exists && !initForce {
			cfgPath, err := config.Path()
			if err != nil {
				return err
			}
			if initLogsDir != "" {
				return fmt.Errorf("config already exists at %s; pass --force to overwrite", cfgPath)
			}
			overwrite, err := confirmOverwrite(cfgPath)
			if err != nil {
				return err
			}
			if !overwrite {
				cmd.Println("aborted")
				return nil
			}
		}

		var cfg config.Config
		if initLogsDir != "" {
			cfg = config.Config{LogsDir: initLogsDir}
		} else {
			cfg, err = promptUserForConfig()
			if err != nil {
				return err
			}
		}

		cfgPath, err := config.WriteConfig(cfg)
		if err != nil {
			return err
		}
		cmd.Printf("Config file initialized at: %s\n", cfgPath)

		return nil
	},
}

func init() {
	initCmd.Flags().StringVar(&initLogsDir, "logs-dir", "", "game logs directory; skips the interactive prompt")
	initCmd.Flags().BoolVar(&initForce, "force", false, "overwrite an existing config without confirmation")
	rootCmd.AddCommand(initCmd)
}

func confirmOverwrite(cfgPath string) (bool, error) {
	var overwrite bool
	confirm := huh.NewConfirm().
		Title(fmt.Sprintf("Config exists at %s — overwrite?", cfgPath)).
		Value(&overwrite)
	if err := confirm.Run(); err != nil {
		return false, fmt.Errorf("unable to get confirmation from input")
	}
	return overwrite, nil
}

func promptUserForConfig() (config.Config, error) {
	var cfg config.Config

	form := huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Enter in your game logs directory").
				Prompt("> ").
				Value(&cfg.LogsDir),
		),
	)

	if err := form.Run(); err != nil {
		return config.Config{}, fmt.Errorf("unable to get config information from input")
	}

	return cfg, nil
}
