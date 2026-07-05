package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"grimoire/core/config"
)

var cfg config.Config

var rootCmd = &cobra.Command{
	Use:          "grimoire",
	Short:        "Grimoire is an agentic video game journaling system",
	Long:         "Grimoire is an agentic video game journaling system",
	SilenceUsage: true, // Runtime errors shouldn't dump the usage block
	PersistentPreRunE: func(cmd *cobra.Command, args []string) error {
		// Commands that must work before a config exists; parents walked so completion's subcommands match
		for c := cmd; c != nil; c = c.Parent() {
			switch c.Name() {
			case "init", "help", "completion":
				return nil
			}
		}

		var err error
		cfg, err = config.GetConfig()
		if err != nil {
			return fmt.Errorf("%w\nYou can initialize or re-initialize the config with `init`", err)
		}

		return nil
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}
