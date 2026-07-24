# Grimoire Spec

Grimoire is a personal game tracker and journaling system: keep a backlog, log what you've played, and write your thoughts about it.

## Design thesis
Grimoire treats personal game tracking as a knowledge-management problem, not a social or analytics problem. The user's library is a private, durable, portable artifact that outlives any tool. Tools are interfaces over the artifact, not the source of truth. This inverts the SaaS model where the platform owns the data; here, the user owns plain files and tools come and go.

## Naming
- Package name: `grimoire-tracker`
- Full name: `grimoire`
- CLI binary: `grim`

## Architecture
- The work splits into two layers: a **core library** that manages the on-disk collection and provider integrations, and one or more **frontends** that present it.
- Initial frontend is a lightweight **CLI**. Pretty output (colors, tables) is fine; a full TUI is explicitly out of scope — it's a middle ground that pays for itself in neither direction.
- A **GUI** may be added later if usage justifies it. Card-grid browsing with cover art is the main motivator; the decision is deferred until the CLI has been used on real data.
- The core lib is designed as if a GUI consumes it from day one:
    - Business logic returns values; it never prints.
    - Errors are returned as typed values, never panicked.
    - No interactive prompts inside the lib. Flows that need user choice (e.g. per-field provider resolution) return a structured request that the frontend renders — numbered prompts in the CLI, a grid in a future GUI.

## Core Interactions
- Add
    - Adding a game means creating a new file for it and populating the file with metadata
- Search
    - Queries configured providers for candidate games by name. Used internally by `add`; also a public command for discovery.
- Refresh
    - Re-runs the provider resolution flow against an existing entry, using its stored `provider_ids`. Applies the same per-field, no-silent-overwrite semantics as `add`.
- Import
    - Bulk-ingests owned/wishlisted games from a storefront (Steam, GOG, Epic, ...) as stub entries. One-way: storefronts are input, the collection is the source of truth.
- Check
    - Validates the file and raises issues with schema alignment or formatting. Also fills in any missing schema fields as `null` so every file always carries the full set of keys.
- View or List
    - Shows the user their collection that can be filtered or sorted by some attributes
- Stats
    - Able to see overall stats about their collection
- Recommend
    - Uses either a local embedding model or LLM calls to recommend the next game to play from unplayed games in the collection

## Config
- App config lives in `~/.config/grimoire/`. Two files, both YAML — reusing the same parser as the collection files.
    - `config.yaml` — collection location, enabled providers, per-provider preferences (e.g. preferred cover source). Safe to commit/share.
    - `keys.yaml` — provider API keys (IGDB/Twitch client ID + secret, Steam web API key, SteamGridDB key, ...). Should be `chmod 600`. Never committed.
- If `keys.yaml` is missing or a given provider's key is absent, that provider is simply disabled for the session — no crash, no nag. The user still gets a working app.
- Manual fallback: when no providers are configured (or all are disabled), `add` falls back to a fully manual flow. The user is prompted to fill each schema field by hand; everything is written as `null` if skipped. The collection still works, it just doesn't get auto-populated.

## Data substrate
- Data is local-first and portable.
- Collection is managed as plain text files that can be easily read or edited in any editor.
- These text files are markdown files with YAML frontmatter.

### Metadata
- One file per game: `games/<slug>.md`. The file is the complete record — single self-contained artifact, fully portable. Binary assets (cover art, etc.) live alongside (e.g. `.grimoire/art/<slug>.jpg`) and are referenced by path.
- Filenames are decorative, not load-bearing. Identity comes from file contents on every operation; the app never indexes by path.
    - At creation, the app generates `<slug>.md` from the title: lowercase, kebab-case, ASCII-folded.
    - On slug collision (e.g. `resident-evil-2` for both 1998 and 2019), the app appends a discriminator (year from `release_date` if available, else `-2`, `-3`, ...).
    - After creation, users may rename freely. Operations like `show`, `open`, `refresh` fuzzy-match against the `title` field, not the filename.
    - `check` flags duplicate `provider_ids` across files to catch accidental copies that share identity.
- Performance: the app does a full directory walk + parse on operations that need the whole collection (`list`, `stats`, `check`, `refresh --all`). At expected sizes (under ~2k entries for most users, ~10k upper bound) this is acceptable on SSD with parallel reads. A lazy index cache is a future optimization, not a v1 concern; since identity is content-derived, the cache is regeneratable from source files and adding it does not change the data model.
- Future: content hashes (e.g. SHA-256 of the file) can serve as durable, content-derived IDs and as a cheap "has this changed?" check for any future index. Not needed for v1, but the design leaves room for it without schema changes.
- Frontmatter is one flat set of fields. The user owns every field and may edit any of them. Providers are a seeding tool, not an owner.
- Every schema field is always present in the frontmatter. Unset values are written as YAML `null`. This makes files self-documenting (the user can see at a glance what fields exist and what's missing) and removes the "is this field absent because unset or unknown to the schema?" ambiguity. `check` will add any missing fields as `null`.
- The app defines a tight, provider-agnostic canonical schema. The schema is deliberately small — heavy or marketing-flavored fields (screenshot lists, store descriptions, alt titles) are excluded to keep the file readable.
- The markdown body is purely free-form. The app does not parse, structure, or validate it. The user writes as much or as little as they want — anything from nothing to a multi-section essay. No heading is required. Conventions may emerge or be imposed later if useful; for v1 it is an open text box.
    - `title` (string)
    - `description` (string)
    - `release_date` (date)
    - `developer` (string)
    - `publisher` (string)
    - `tags` (list of strings — covers genre and any user-defined labels)
    - `cover` (path)
    - `log` (map) — your playthrough, separated from the game's static facts
        - `status` (enum: `backlog | playing | finished | abandoned | on-hold`)
        - `rating` (integer, 1–5)
        - `played_platform` (string)
        - `started` (date)
        - `finished` (date)
        - `achievement_percent` (integer, 0–100)
        - `hours_played` (number) — total time spent. Provider-fillable (e.g. Steam playtime); user-editable for non-Steam games.
        - `revisit` (boolean) — "I want to play this again." Independent of `status`. Note: `started`/`finished` reflect the most recent playthrough; replaying overwrites them. Schema may grow a playthroughs list later if this proves limiting.
    - `provider_ids` (map) — provider lookup keys (`igdb`, `steam`, `steamgriddb`, ...)
- Per-source adapters normalize each provider's response into the canonical schema. Adding a new provider means writing a new adapter.
- Providers cover different subsets of the canonical schema — none is expected to populate every field. Each adapter declares which canonical fields it supports, and the resolver only considers a provider for fields it actually covers. Concretely: IGDB covers most static facts (title, description, release_date, developer, publisher, tags, cover); Steam covers playtime, achievements, and store-side cover; SteamGridDB covers cover only.
- Provider IDs (`igdb_id`, `steam_appid`, ...) are stored in frontmatter purely as lookup keys for re-querying. They are not "metadata about the game."
- Resolution flow: on add, the app fans out queries across enabled providers and presents candidates to the user at the field level. For each canonical field, the user only sees candidates from providers that cover that field. The user picks the value they prefer per field (e.g. IGDB's description, SteamGridDB's cover, Steam's hours_played). Chosen values are written into the frontmatter; from that point on they are just values, not "owned" by any provider.
- Refresh semantics: provider updates only happen on `add` or an explicit `refresh` action — never automatically. Refresh must never silently overwrite user data.
    - `null` field → fill from provider, no prompt.
    - Non-null field → show what the provider says and let the user accept / append / ignore. Always per-field, always explicit. Applies to every field, including `achievement_percent`.

### Example file
```markdown
---
title: Elden Ring
description: A vast open-world action RPG set in the Lands Between...
release_date: 2022-02-25
developer: FromSoftware
publisher: Bandai Namco Entertainment
tags:
  - action-rpg
  - open-world
  - soulslike
cover: .grimoire/art/elden-ring.jpg

log:
  status: finished
  rating: 5
  played_platform: Steam Deck
  started: 2024-06-12
  finished: 2024-08-03
  achievement_percent: 87
  hours_played: 132
  revisit: false

provider_ids:
  igdb: 119133
  steam: 1245620
---

Beat Margit on the third try after respeccing into faith. The Lands Between are the best open world I've explored — every horizon hides something. Patches still gets me every time.
```

### Example file — fresh backlog entry (all unset fields written as null)
```markdown
---
title: Hollow Knight Silksong
description: null
release_date: null
developer: Team Cherry
publisher: Team Cherry
tags: []
cover: null

log:
  status: backlog
  rating: null
  played_platform: null
  started: null
  finished: null
  achievement_percent: null
  hours_played: null
  revisit: false

provider_ids:
  igdb: null
  steam: 1030300
---
```

