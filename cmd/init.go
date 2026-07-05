package cmd

import (
	"fmt"
	"grimoire/core/config"

	"github.com/charmbracelet/huh"
	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Init Grimoire config",
	Long:  "Init Grimoire config",
	RunE: func(cmd *cobra.Command, args []string) error {
		cfg, err := promptUserForConfig()
		if err != nil {
			return err
		}

		cfgPath, err := config.WriteConfig(cfg)
		if err != nil {
			return err
		}
		fmt.Printf("Config file initialized at: %s\n", cfgPath)

		return nil
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
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
		return config.Config{}, fmt.Errorf("Unable to get config information from input")
	}

	return cfg, nil
}
