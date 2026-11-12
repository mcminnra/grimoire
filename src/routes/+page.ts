import { redirect } from "@sveltejs/kit";

/** Sends the root route to the default Games page. */
export function load() {
  redirect(307, "/games");
}
