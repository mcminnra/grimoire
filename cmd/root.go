/*
Copyright © 2026 NAME HERE <EMAIL ADDRESS>
*/
package cmd

import (
	"os"
	"path/filepath"

	"github.com/spf13/cobra"
)

var cfgFile string

var rootCmd = &cobra.Command{
	Use:   "grimoire",
	Short: "A journaling system for video games",
	Long:  `Grimoire is a cli journaling system designed to capture your thoughts about the video games you play and their status.`,
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

func init() {
	home, _ := os.UserHomeDir()
	defaultCfgFilepath := filepath.Join(home, ".config/grimoire/config.toml")
	rootCmd.PersistentFlags().StringVar(
		&cfgFile,
		"config",
		defaultCfgFilepath,
		"config filepath",
	)
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}
