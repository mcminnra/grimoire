import { error } from "@sveltejs/kit";
import { loadGameLogs } from "$lib/log";

// Tauri doesn't have a Node.js server to do proper SSR
export const ssr = false;

/** Loads all game logs once on boot */
export async function load() {
  try {
    const { logs, failed } = await loadGameLogs();
    return { logs, failed };
  } catch (e) {
    error(500, `Failed to load game logs: ${e}`);
  }
}
