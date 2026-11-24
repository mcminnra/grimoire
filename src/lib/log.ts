import { invoke } from "@tauri-apps/api/core";

// NOTE: These types mirror the serialized Rust types in `src-tauri/src/parse/` (log.rs, error.rs, parse.rs). Keep them in sync by hand.

/** Every status the backend accepts, in display order. */
export const STATUSES = [
  "backlog",
  "playing",
  "played",
  "abandoned",
  "completed",
  "completed+",
  "mastered",
] as const;

/** Where a game sits in the user's play history. */
export type Status = (typeof STATUSES)[number];

/** A 1 to 5 rating. Mirrors the `Rating` value object. */
export type Rating = 1 | 2 | 3 | 4 | 5;

/** Why a log failed to load. Mirrors `LogLoadError::kind()`. */
export type LogLoadErrorKind =
  | "io"
  | "missingFrontmatter"
  | "unclosedFrontmatter"
  | "parse";

/** Mirrors `LogLoadError`. `message` may span several lines with a source snippet. */
export interface LogLoadError {
  kind: LogLoadErrorKind;
  message: string;
}

/** A log file that failed to load. */
export interface FailedLog {
  fileName: string;
  /** Full file contents. Null only when the file couldn't be read at all. */
  source: string | null;
  error: LogLoadError;
}

/** Play history from the `log:` frontmatter block. Dates are `YYYY-MM-DD`. */
export interface LogDetails {
  status: Status | null;
  rating: Rating | null;
  playedPlatform: string | null;
  started: string | null;
  finished: string | null;
  achievementPercent: number | null;
  hoursPlayed: number | null;
  revisit: boolean | null;
}

/**
 * External provider IDs from the `provider_ids:` frontmatter block.
 * Rust sends `u64`, safe here while IDs stay under `Number.MAX_SAFE_INTEGER`.
 */
export interface ProviderIds {
  steam: number | null;
  igdb: number | null;
}

/** A fully valid game log. `fileName` is its unique key. */
export interface Log {
  fileName: string;
  title: string;
  tags: string[] | null;
  details: LogDetails;
  providerIds: ProviderIds;
  body: string;
}

/** Outcome of loading the log directory. */
export interface LogLoadStatus {
  logs: Log[];
  /** Files that failed at any stage: reading, splitting, or parsing (including value checks). */
  failed: FailedLog[];
}

/**
 * Loads every game log from the backend.
 *
 * @returns Valid logs and failed files, each sorted by file name.
 * @throws The backend's message if the log directory cannot be listed.
 */
export function loadGameLogs(): Promise<LogLoadStatus> {
  return invoke<LogLoadStatus>("load_game_logs");
}
