# Grimoire CLI

The CLI is the initial frontend over the core library. See [overview.md](./overview.md) for the data model, schema, and provider model.

Conventions:
- Commands accepting `[name]` fuzzy-match against existing entries.
- Interactive prompts use a Rust forms library (likely `inquire` or `dialoguer`); rendered markdown uses a Rust terminal-markdown renderer (likely `termimad`).
- All commands operate on the collection path configured in `~/.config/grimoire/config.yaml`.

## Commands

- **grim init**
    - Initializes config at `~/.config/grimoire/config.yaml` and `keys.yaml`.
    - Prompts for API keys for enabled providers (IGDB, Steam, SteamGridDB) and any LLM keys used by `recommend` (e.g. Anthropic, OpenAI).
    - Asks for the grimoire directory location (default `~/grimoire/`).
    - Idempotent — re-running edits config in place rather than overwriting.

- **grim add [name]**
    - Adds a game to the grimoire by creating a new file under the configured collection path.
    - Normalizes the name by querying configured search providers (IGDB, Steam) to confirm the game exists.
    - For each canonical field, gathers candidates only from providers that support that field (see overview.md for the per-provider coverage map). Fields no provider covers are left `null` for the user to fill.
    - Falls back to a fully manual flow when no provider returns a match (fan games, ROM hacks, mods, disabled providers).
    - Interactive form for `[log]` fields (status, rating, played_platform, hours_played, started, finished, ...) and `tags`.

- **grim refresh [name]**
    - Re-runs the provider resolution flow against an existing game file.
    - Uses stored `provider_ids` to query providers directly (no name search needed).
    - Applies the same refresh semantics as `add` (see overview.md): `null` fields are filled silently, non-null fields prompt the user per field to accept / append / ignore. User data is never silently overwritten.
    - Fuzzy-matches `[name]` against existing files. Pass `--all` to refresh every file in the collection.

- **grim show [name]**
    - Displays a game's full record in the terminal. Read-only.
    - Renders frontmatter as a clean header (status, rating, hours, dates, platform, tags).
    - Renders the free-form markdown body with a terminal markdown renderer.
    - Inlines provider-seeded fields (description, developer, publisher, release date, cover preview where supported) directly from the file — there is no separate cache.
    - Fuzzy-matches `[name]` against existing files.
    - For editing, use `grim open`.

- **grim open [name]**
    - Opens the game's markdown file in `$EDITOR`.
    - Fuzzy-matches `[name]` against existing files.
    - This is how editing happens — grimoire does not ship its own editor.

- **grim search [query]**
    - Searches configured providers (IGDB, Steam) for candidate games by name.
    - Backed by a pluggable search-providers abstraction in the core lib.
    - Used internally by `grim add`; also available as a public command for discovery and to feed a quick `grim add` of a chosen result.
    - BYOK for providers that require it; providers without configured keys are skipped silently.

- **grim list**
    - Displays the collection in a pretty table.
    - Filter flags: `--status`, `--played-platform`, `--tag`, `--rating-min`, `--rating-max`.
    - Sort flags: `--sort {title,rating,hours_played,added,finished}`, `--reverse`.

- **grim check**
    - Lints existing game files.
    - Fills any missing schema keys with `null` so every file always carries the full set.
    - Flags schema violations (wrong types, out-of-range values, unknown enum values).
    - Flags suspicious cross-field combinations (e.g. `status: finished` without a `finished` date, `rating` set on a `backlog` game).

- **grim import [--steam | --gog | --epic | ...]**
    - Bulk-imports owned and wishlisted games from a storefront.
    - Creates stub markdown files with minimal frontmatter (`status: backlog`, storefront ID in `provider_ids`) for games not already in the grimoire.
    - Idempotent — re-running skips already-imported games (matched by storefront ID).
    - One-way only: storefronts are input, grimoire is the source of truth.

- **grim stats**
    - Computed summary: % completed, total hours played, library breakdown by status / played_platform / tag, average rating, etc.
    - No LLM involved; pure aggregation over frontmatter.
