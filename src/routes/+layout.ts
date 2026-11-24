import { error } from "@sveltejs/kit";
import { logStore } from "$lib/logStore.svelte";

// Tauri doesn't have a Node.js server to do proper SSR
export const ssr = false;

/** Fills the log store from backend state once on boot. Later changes arrive per log. */
export async function load() {
  try {
    await logStore.load();
  } catch (e) {
    error(500, `Failed to load game logs: ${e}`);
  }
}
