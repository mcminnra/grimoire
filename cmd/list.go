package cmd

import (
	"fmt"
	"grimoire/core/log"
	"os"
	"path/filepath"
	"strings"

	"github.com/adrg/frontmatter"
	"github.com/spf13/cobra"
)

var listCmd = &cobra.Command{
	Use:   "list",
	Short: "List games in your grimoire",
	Long:  "List games in your grimoire",
	RunE: func(cmd *cobra.Command, args []string) error {
		if err := listGames(); err != nil {
			return err
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(listCmd)
}

func listGames() error {
	entries, err := os.ReadDir(cfg.LogsDir)
	if err != nil {
		return fmt.Errorf("Unable to read log directory %s", cfg.LogsDir)
	}

	// List games
	var inProgressGames []log.Game
	for _, entry := range entries {
		fmt.Printf("Name: %s\n", entry.Name())

		gameFilePath := filepath.Join(cfg.LogsDir, entry.Name())
		content, err := os.ReadFile(gameFilePath)
		if err != nil {
			return fmt.Errorf("Unable to read log %s: %w", gameFilePath, err)
		}

		var game log.Game
		rest, err := frontmatter.Parse(strings.NewReader(string(content)), &game)
		if err != nil {
			return fmt.Errorf("Unable to parse log %s: %w", gameFilePath, err)
		}

		fmt.Printf("%s // %s\n", game.Title, game.Log.Status)
		fmt.Println("---")
		fmt.Printf("%v\n\n", strings.TrimSpace(string(rest)))
		if game.Log.Status == "playing" {
			inProgressGames = append(inProgressGames, game)
		}
	}

	fmt.Println("=== Playing Games ===")
	for _, game := range inProgressGames {
		fmt.Printf("%s\n", game.Title)
	}

	return nil
}
