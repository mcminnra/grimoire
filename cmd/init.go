package cmd

import (
	"fmt"
	"grimoire/core"

	"github.com/charmbracelet/huh"
	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Init Grimoire config",
	Long:  "Init Grimoire config",
	RunE: func(cmd *cobra.Command, args []string) error {
		config, err := promptUserForConfig()
		if err != nil {
			return err
		}

		configPath, err := core.WriteConfig(config)
		if err != nil {
			return err
		}
		fmt.Printf("Config file initialized at: %s\n", configPath)

		return nil
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}

func promptUserForConfig() (core.Config, error) {
	var config core.Config

	form := huh.NewForm(
		huh.NewGroup(
			huh.NewInput().
				Title("Enter in your games log directory").
				Prompt("> ").
				Value(&config.LogsDir),
		),
	)

	if err := form.Run(); err != nil {
		return core.Config{}, fmt.Errorf("Unable to get config information from input")
	}

	return config, nil
}
