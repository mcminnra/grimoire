/*
1. Read file into RawDocument
2. Parse RawDocument into domain Object ParsedGame, noting issues
3. On edit, we re-read file into a new RawDocument, updating only the fields the edit touch, write back
 */

// === Document

export type RawDocument = {
  frontmatter: Record<string, unknown> // generic — unknown user keys survive untouched
  body: string // markdown after the frontmatter
}

// === Domain

export type Collection = Record<string, GameEntry> // Mapped by filepath

export type GameEntry = {
  game: Game // best-effort strict; invalid fields coerced to null
  field_issues: Record<string, FieldIssue> // what didn't validate, keyed by field string
  violations: ConstraintViolation[] // violates some association between fields
}

// NOTE: on view+edit, we overlay these on the UX with a warning
type FieldIssue = {
  field: string // "log.rating"
  value: unknown // 7
  message: string // "rating must be 1–5"
}

// NOTE: on view+edit, we show these in a dialog
type ConstraintViolation = {
  rule: string // stable id: "finished-requires-completed-status"
  fields: string[] // participants: ["log.status", "log.finished"] — same path vocab as FieldIssue keys
  message: string // "status is 'completed' but no finished date is set"
}

export type Game = {
  filepath: string
  filename: string
  body: string
} & GameAttributes

type GameAttributes = {
  log: Log
  metadata: Metadata
  provider_ids: ProviderIds
}

type Log = {
  status: 'backlog' | 'playing' | 'played' | 'abandoned' | 'completed' | 'completed+' | 'mastered'
  rating: 1 | 2 | 3 | 4 | 5 | null
  played_platform: string | null
  started: Date | null
  finished: Date | null
  achievement_percent: number | null
  hours_played: number | null
  revisit: boolean
}

type Metadata = {
  title: string
  description: string | null
  release_date: Date | null
  developer: string | null
  publisher: string | null
  tags: string[] | null
}

type ProviderIds = {
  steam: {
    appid: number | null
  }
  igdb: {
    id: number | null
  }
}

// === Edit / Write

// Deep-partial that STOPS at atomic values — arrays & Dates are replaced wholesale, never merged into
// We create caveats for Date and Array so the partial doesn't disambiguate the Date and Array objects
type DeepPartial<T> = T extends Date
  ? T
  : T extends Array<unknown>
    ? T
    : T extends object
      ? { [K in keyof T]?: DeepPartial<T[K]> }
      : T

// Sparse patch over what we own. Present key = overwrite (null clears); absent key = leave untouched
// Mirrors RawDocument's { frontmatter, body } split so the future deep merge is two lines
// NOTE: filename is addressing, not content — it stays a sibling at the IPC layer, never inside the patch
export type EditPatch = DeepPartial<{
  frontmatter: GameAttributes
  body: string
}>
