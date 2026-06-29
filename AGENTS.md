# Grimoire

## What this is
Grimoire is a CLI-based personal game tracker and journaling tool. It helps the user maintain a collection of plain-text files (one per game) for games played and games to play. The design treats the collection as a private, durable, portable knowledge artifact — tools are interfaces over the files, not the source of truth.

## Design docs
Before answering questions about behavior, schema, commands, or architecture, consult these. They are authoritative; this file is a pointer.
- `docs/design/overview.md` — design thesis, architecture (lib + frontends), config layout, full metadata schema, provider model, refresh semantics, slug/filename rules, examples.
- `docs/design/cli.md` — `grim` command surface. Cross-references overview.md for the data model; do not duplicate that here.

When the user asks about a design decision, look first in these docs. When they ask you to change a design decision, update the relevant doc.

## How to run
- run: `go run main.go`
- build executable: `go build`
- format code: `go fmt`
- tidy go.mod and go.sum: `go mod tidy`

