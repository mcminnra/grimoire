package cmd

import (
	"bytes"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// executeInit runs `grim init` with args against the package-level rootCmd and captures output
func executeInit(t *testing.T, args ...string) (string, error) {
	t.Helper()
	// NOTE: rootCmd/initCmd are package-level; reset flag state so tests don't leak into each other
	t.Cleanup(func() {
		initLogsDir = ""
		initForce = false
	})

	buf := new(bytes.Buffer)
	rootCmd.SetOut(buf)
	rootCmd.SetErr(buf)
	rootCmd.SetArgs(append([]string{"init"}, args...))
	err := rootCmd.Execute()
	return buf.String(), err
}

func configFilePath(t *testing.T, home string) string {
	t.Helper()
	return filepath.Join(home, ".config", "grimoire", "config")
}

func TestInitHappyPath(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)
	logsDir := t.TempDir()

	out, err := executeInit(t, "--logs-dir", logsDir)
	if err != nil {
		t.Fatalf("init failed: %v", err)
	}

	cfgPath := configFilePath(t, home)
	b, err := os.ReadFile(cfgPath)
	if err != nil {
		t.Fatalf("expected config file at %s: %v", cfgPath, err)
	}
	if !strings.Contains(string(b), "logs_dir") || !strings.Contains(string(b), logsDir) {
		t.Errorf("config file missing logs_dir %q, got:\n%s", logsDir, b)
	}
	if !strings.Contains(out, cfgPath) {
		t.Errorf("output should name the config path %q, got: %q", cfgPath, out)
	}
}

func TestInitInvalidLogsDir(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)

	_, err := executeInit(t, "--logs-dir", filepath.Join(home, "does-not-exist"))
	if err == nil {
		t.Fatal("expected error for nonexistent logs dir")
	}
	if _, statErr := os.Stat(configFilePath(t, home)); !os.IsNotExist(statErr) {
		t.Error("no config file should be created on validation failure")
	}
}

func TestInitExistingConfigWithoutForce(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)
	logsDir := t.TempDir()

	if _, err := executeInit(t, "--logs-dir", logsDir); err != nil {
		t.Fatalf("first init failed: %v", err)
	}
	original, err := os.ReadFile(configFilePath(t, home))
	if err != nil {
		t.Fatalf("reading config after first init: %v", err)
	}

	otherDir := t.TempDir()
	_, err = executeInit(t, "--logs-dir", otherDir)
	if err == nil {
		t.Fatal("expected error when config already exists without --force")
	}
	if !strings.Contains(err.Error(), "--force") {
		t.Errorf("error should mention --force, got: %v", err)
	}

	after, err := os.ReadFile(configFilePath(t, home))
	if err != nil {
		t.Fatalf("reading config after refused init: %v", err)
	}
	if !bytes.Equal(original, after) {
		t.Error("config file should be unchanged after refused init")
	}
}

func TestInitExistingConfigWithForce(t *testing.T) {
	home := t.TempDir()
	t.Setenv("HOME", home)

	if _, err := executeInit(t, "--logs-dir", t.TempDir()); err != nil {
		t.Fatalf("first init failed: %v", err)
	}

	otherDir := t.TempDir()
	if _, err := executeInit(t, "--logs-dir", otherDir, "--force"); err != nil {
		t.Fatalf("forced re-init failed: %v", err)
	}

	b, err := os.ReadFile(configFilePath(t, home))
	if err != nil {
		t.Fatalf("reading config after forced init: %v", err)
	}
	if !strings.Contains(string(b), otherDir) {
		t.Errorf("config should contain new logs dir %q, got:\n%s", otherDir, b)
	}
}
