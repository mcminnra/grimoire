import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

// NOTE: These types mirror the serialized Rust types in `src-tauri/src/parse/` (log.rs, error.rs) and the commands in `src-tauri/src/lib.rs`. Keep them in sync by hand.

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

/** Why a log failed to load. Mirrors `LogError::kind()`. */
export type LogErrorKind =
  | "io"
  | "missingFrontmatter"
  | "unclosedFrontmatter"
  | "parse";

/** Mirrors `LogError`. `message` may span several lines with a source snippet. */
export interface LogError {
  kind: LogErrorKind;
  message: string;
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

/** Validated frontmatter of a log. Mirrors `LogMetadata`. */
export interface LogMetadata {
  title: string;
  tags: string[] | null;
  details: LogDetails;
  providerIds: ProviderIds;
}

/** A log as the backend exposes it. Mirrors `LogView`. `fileName` is its unique key. */
export interface LogView {
  fileName: string;
  /** Full file contents. Null only when the file couldn't be read at all. */
  source: string | null;
  /** Parsed frontmatter. Null when any stage of loading failed. */
  metadata: LogMetadata | null;
  /** Everything after the frontmatter. Null when the file couldn't be read or split. */
  body: string | null;
  error: LogError | null;
}

/** A log that loaded cleanly, so its metadata and body are always present. */
export type ValidLog = LogView & {
  metadata: LogMetadata;
  body: string;
  error: null;
};

/**
 * Narrows a log to one that loaded cleanly.
 *
 * @param log Log to check.
 * @returns True when the log has no error.
 */
export function isValidLog(log: LogView): log is ValidLog {
  return log.error === null && log.metadata !== null && log.body !== null;
}

/** Event the backend emits with a log's `fileName` as payload whenever that log changes. */
export const LOG_CHANGED = "log-changed";

/**
 * Fetches every log from the backend. The first call loads the log directory.
 *
 * @returns All logs, valid and failed, sorted by file name.
 * @throws The backend's message if the log directory cannot be loaded.
 */
export function getLogs(): Promise<LogView[]> {
  return invoke<LogView[]>("get_logs");
}

/**
 * Fetches one log from the backend.
 *
 * @param fileName Key of the log to fetch.
 * @returns The log, or null if no log has that file name.
 * @throws The backend's message if the log directory cannot be loaded.
 */
export function getLog(fileName: string): Promise<LogView | null> {
  return invoke<LogView | null>("get_log", { fileName });
}

/**
 * Writes new source to a log's file. The backend re-parses it and emits `LOG_CHANGED`.
 *
 * @param fileName Key of the log to save.
 * @param source Full new file contents.
 * @returns The re-parsed log. Invalid frontmatter still saves and comes back with `error` set.
 * @throws The backend's message if the log is unknown or the write fails.
 */
export function saveLog(fileName: string, source: string): Promise<LogView> {
  return invoke<LogView>("save_log", { fileName, source });
}

/**
 * Subscribes to backend log changes.
 *
 * @param handler Called with the changed log's file name.
 * @returns A function that unsubscribes. Await it before calling.
 */
export function onLogChanged(handler: (fileName: string) => void): Promise<UnlistenFn> {
  return listen<string>(LOG_CHANGED, (event) => handler(event.payload));
}
