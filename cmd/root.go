package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"grimoire/core/config"
)

var cfg config.Config

var rootCmd = &cobra.Command{
	Use:   "grimoire",
	Short: "Grimoire is a agentic video game journaling system",
	Long:  "Grimoire is a agentic video game journaling system",
	PersistentPreRunE: func(cmd *cobra.Command, args []string) error {
		var err error
		cfg, err = config.GetConfig()
		if err != nil && cmd.CalledAs() != "init" {
			fmt.Println(err)
			fmt.Println("You can initialize or re-initialize the config with `init`")
			os.Exit(1)
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
