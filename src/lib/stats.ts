import { STATUSES, type Status, type ValidLog } from "$lib/log";

/** Number of logs with a given status. `status` is null for logs without one. */
export interface StatusCount {
  status: Status | null;
  count: number;
}

/** Average rating across the rated logs carrying one tag. */
export interface TagRating {
  tag: string;
  average: number;
  /** How many rated logs the average is over. */
  count: number;
}

/**
 * Counts logs per status, in `STATUSES` order, with a trailing null entry for unset statuses.
 *
 * @param logs Logs to count.
 * @returns One entry per status, including zero counts.
 */
export function statusCounts(logs: ValidLog[]): StatusCount[] {
  const counts = new Map<Status | null, number>();
  for (const log of logs) {
    const status = log.metadata.details.status;
    counts.set(status, (counts.get(status) ?? 0) + 1);
  }
  return [...STATUSES, null].map((status) => ({
    status,
    count: counts.get(status) ?? 0,
  }));
}

/**
 * Averages ratings per tag over logs that have both a rating and tags.
 *
 * @param logs Logs to aggregate. Unrated logs are ignored.
 * @returns One entry per tag, highest average first, ties broken by count then name.
 */
export function averageRatingByTag(logs: ValidLog[]): TagRating[] {
  const totals = new Map<string, { sum: number; count: number }>();
  for (const log of logs) {
    const rating = log.metadata.details.rating;
    if (rating == null) continue;
    for (const tag of new Set(log.metadata.tags ?? [])) {
      const total = totals.get(tag) ?? { sum: 0, count: 0 };
      total.sum += rating;
      total.count += 1;
      totals.set(tag, total);
    }
  }
  return [...totals]
    .map(([tag, { sum, count }]) => ({ tag, average: sum / count, count }))
    .sort(
      (a, b) =>
        b.average - a.average || b.count - a.count || a.tag.localeCompare(b.tag),
    );
}
