import { css } from "styled-system/css";
import type { Rating, Status } from "$lib/log";

// NOTE: Panda only emits classes it can see statically, so every style is a literal css() call.

/** Display name for each status. */
export const STATUS_LABELS: Record<Status, string> = {
  backlog: "Backlog",
  playing: "Playing",
  played: "Played",
  abandoned: "Abandoned",
  completed: "Completed",
  "completed+": "Completed+",
  mastered: "Mastered",
};

/** Text color class for each status. */
export const STATUS_CLASSES: Record<Status, string> = {
  backlog: css({ color: "gray.400" }),
  playing: css({ color: "cyan.500" }),
  played: css({ color: "blue.500" }),
  abandoned: css({ color: "red.500" }),
  completed: css({ color: "green.500" }),
  "completed+": css({ color: "amber.400" }),
  mastered: css({
    bgGradient: "to-r",
    gradientFrom: "blue.600",
    gradientVia: "green.500",
    gradientTo: "amber.400",
    bgClip: "text",
    color: "transparent",
  }),
};

/** Text color class for each 1 to 5 rating. */
export const RATING_CLASSES: Record<Rating, string> = {
  1: css({ color: "#ff0a0a" }),
  2: css({ color: "#ffa500" }),
  3: css({ color: "#FFEA00" }),
  4: css({ color: "#85e62c" }),
  5: css({ color: "#209c05" }),
};
