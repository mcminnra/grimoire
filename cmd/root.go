package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"

	"grimoire/core"
)

var cfg core.Config

var rootCmd = &cobra.Command{
	Use:   "grimoire",
	Short: "Grimoire is a video game journaling system",
	Long:  "Grimoire is a video game journaling system",
	PersistentPreRunE: func(cmd *cobra.Command, args []string) error {
		var err error
		cfg, err = core.InitConfig()
		if err != nil {
			fmt.Println(err)
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
