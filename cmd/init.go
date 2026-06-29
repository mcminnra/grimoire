package cmd

import (
	"fmt"

	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "Init Grimoire config",
	Long:  "Init Grimoire config",
	RunE: func(cmd *cobra.Command, args []string) error {
		if err := createConfig(); err != nil {
			return err
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}

func createConfig() error {
	fmt.Println("TODO Init")
	return nil
}
