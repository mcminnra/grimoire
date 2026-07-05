package cmd

import (
	"fmt"
	"grimoire/core/log"
	"os"
	"path/filepath"
	"strconv"
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

	// Get games obj
	var games []log.Game
	for _, entry := range entries {
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
		game.Review = strings.TrimSpace(string(rest))

		games = append(games, game)
	}

	// Output
	fmt.Println("=== Games ===")
	for _, game := range games {
		rating := "?"
		if game.Log.Rating != nil {
			rating = strconv.Itoa(*game.Log.Rating)
		}
		fmt.Printf("[%s]\t[%s/5] %s\n", game.Log.Status, rating, game.Title)
	}

	fmt.Println("\n=== Playing Games ===")
	for _, game := range games {
		if game.Log.Status == "playing" {
			rating := "?"
			if game.Log.Rating != nil {
				rating = strconv.Itoa(*game.Log.Rating)
			}
			fmt.Printf("[%s]\t[%s/5] %s\n", game.Log.Status, rating, game.Title)
		}

	}

	return nil
}
