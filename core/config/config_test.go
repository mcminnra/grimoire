package config

import (
	"testing"
)

// TestExpandHome covers path resolution: ~/ prefix, $HOME expansion, and
// paths that should be left untouched. HOME is pinned so results are
// deterministic regardless of the machine running the test.
func TestExpandHome(t *testing.T) {
	const home = "/home/tester"

	// Pin HOME for both os.ExpandEnv("$HOME") and os.UserHomeDir().
	t.Setenv("HOME", home)

	cases := []struct {
		name string
		in   string
		want string
	}{
		{"tilde prefix", "~/games", home + "/games"},
		{"absolute path untouched", "/var/lib/games", "/var/lib/games"},
		{"home env var", "$HOME/games", home + "/games"},
		{"bare tilde not expanded", "~", "~"}, // NOTE: documents current limitation
		{"empty stays empty", "", ""},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			got, err := expandHome(tc.in)
			if err != nil {
				t.Fatalf("expandHome(%q) returned error: %v", tc.in, err)
			}

			if got != tc.want {
				t.Errorf("expandHome(%q) = %q, want %q", tc.in, got, tc.want)
			}
		})
	}
}
