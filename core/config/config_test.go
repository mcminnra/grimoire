package config

import (
	"errors"
	"os"
	"path/filepath"
	"strings"
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

// TestWriteGetRoundTrip covers the write→read cycle returning an equal Config.
func TestWriteGetRoundTrip(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)
	logsDir := t.TempDir()

	want := Config{LogsDir: logsDir}
	if _, err := WriteConfig(want); err != nil {
		t.Fatalf("WriteConfig returned error: %v", err)
	}

	got, err := GetConfig()
	if err != nil {
		t.Fatalf("GetConfig returned error: %v", err)
	}
	if got != want {
		t.Errorf("GetConfig = %+v, want %+v", got, want)
	}
}

// TestWriteConfigNormalizesTilde covers ~/ expansion persisting to disk.
func TestWriteConfigNormalizesTilde(t *testing.T) {
	// NOTE: HOME is pinned to a temp dir, so ~/logs expands to <tempdir>/logs — never the real home
	home := t.TempDir()
	t.Setenv("HOME", home)
	logsDir := filepath.Join(home, "logs")
	if err := os.Mkdir(logsDir, 0o755); err != nil {
		t.Fatal(err)
	}

	cfgPath, err := WriteConfig(Config{LogsDir: "~/logs"})
	if err != nil {
		t.Fatalf("WriteConfig returned error: %v", err)
	}

	b, err := os.ReadFile(cfgPath)
	if err != nil {
		t.Fatalf("reading written config: %v", err)
	}
	if strings.Contains(string(b), "~") {
		t.Errorf("config on disk should contain the expanded path, got:\n%s", b)
	}
	if !strings.Contains(string(b), logsDir) {
		t.Errorf("config on disk should contain %q, got:\n%s", logsDir, b)
	}
}

func TestGetConfigMissingFile(t *testing.T) {
	t.Setenv("HOME", t.TempDir())

	_, err := GetConfig()
	if err == nil {
		t.Fatal("expected error when config file does not exist")
	}
	var cfgErr *ConfigError
	if !errors.As(err, &cfgErr) {
		t.Fatalf("expected *ConfigError, got %T: %v", err, err)
	}
	if cfgErr.Op != "read" {
		t.Errorf("Op = %q, want %q", cfgErr.Op, "read")
	}
}

func TestGetConfigMalformedTOML(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)

	cfgPath, err := Path()
	if err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(filepath.Dir(cfgPath), 0o755); err != nil {
		t.Fatal(err)
	}
	if err := os.WriteFile(cfgPath, []byte("logs_dir = [not toml"), 0o644); err != nil {
		t.Fatal(err)
	}

	_, err = GetConfig()
	if err == nil {
		t.Fatal("expected error for malformed TOML")
	}
	var cfgErr *ConfigError
	if !errors.As(err, &cfgErr) {
		t.Fatalf("expected *ConfigError, got %T: %v", err, err)
	}
	if cfgErr.Op != "parse" {
		t.Errorf("Op = %q, want %q", cfgErr.Op, "parse")
	}
}

// TestValidateConfigNotADirectory covers logs_dir pointing at a file.
func TestValidateConfigNotADirectory(t *testing.T) {
	home := t.TempDir()
	filePath := filepath.Join(home, "not-a-dir")
	if err := os.WriteFile(filePath, []byte("x"), 0o644); err != nil {
		t.Fatal(err)
	}

	err := ValidateConfig(Config{LogsDir: filePath})
	if err == nil {
		t.Fatal("expected error when logs_dir is a file")
	}
	if !strings.Contains(err.Error(), "not a directory") {
		t.Errorf("error should mention 'not a directory', got: %v", err)
	}
}

func TestExists(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)

	exists, err := Exists()
	if err != nil {
		t.Fatalf("Exists returned error: %v", err)
	}
	if exists {
		t.Error("Exists = true before any config written")
	}

	if _, err := WriteConfig(Config{LogsDir: t.TempDir()}); err != nil {
		t.Fatalf("WriteConfig returned error: %v", err)
	}

	exists, err = Exists()
	if err != nil {
		t.Fatalf("Exists returned error: %v", err)
	}
	if !exists {
		t.Error("Exists = false after config written")
	}
}
