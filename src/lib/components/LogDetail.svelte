<!--
@component
Full view of one game log: frontmatter details, tags, and body.
-->
<script lang="ts">
    import { RATING_CLASSES, STATUS_CLASSES, STATUS_LABELS } from "$lib/logDisplay";
    import type { Log } from "$lib/log";
    import { css } from "styled-system/css";
    import { flex, grid } from "styled-system/patterns";

    let { log }: { log: Log } = $props();

    /** One row of the details panel. `valueClass` optionally colors the value. */
    interface DetailRow {
        name: string;
        value: string | number | null;
        valueClass?: string;
    }

    /** Frontmatter fields shown in the details panel, skipping empty values. */
    const details = $derived.by(() => {
        const { status, rating, achievementPercent, revisit } = log.details;
        const rows: DetailRow[] = [
            {
                name: "Status",
                value: status && STATUS_LABELS[status],
                valueClass: status ? STATUS_CLASSES[status] : undefined,
            },
            {
                name: "Rating",
                value: rating && `${rating} / 5`,
                valueClass: rating ? RATING_CLASSES[rating] : undefined,
            },
            { name: "Platform", value: log.details.playedPlatform },
            { name: "Started", value: log.details.started },
            { name: "Finished", value: log.details.finished },
            { name: "Hours played", value: log.details.hoursPlayed },
            {
                name: "Achievements",
                value: achievementPercent != null ? `${achievementPercent}%` : null,
            },
            { name: "Revisit", value: revisit ? "Yes" : null },
        ];
        return rows.filter((row) => row.value != null);
    });

    const label = css({ fontSize: "xs", color: "bone.300" });
</script>

<h1 class={css({ fontSize: "3xl", fontWeight: "bold", mb: "4" })}>
    {log.title}
</h1>

<!-- Details -->
{#if details.length}
    <dl class={grid({ columns: { base: 2, xl: 4 }, gap: "4", mb: "6" })}>
        {#each details as { name, value, valueClass } (name)}
            <div class={flex({ direction: "column", gap: "0.5" })}>
                <dt class={label}>{name}</dt>
                <dd class={valueClass}>{value}</dd>
            </div>
        {/each}
    </dl>
{/if}

{#if log.tags?.length}
    <div class={flex({ gap: "2", wrap: "wrap", mb: "6" })}>
        {#each log.tags as tag (tag)}
            <span
                class={css({
                    px: "2",
                    py: "0.5",
                    fontSize: "xs",
                    bg: "arcane.800",
                })}>{tag}</span
            >
        {/each}
    </div>
{/if}

<!-- Body -->
<!-- TODO(agent): render markdown instead of raw text -->
<article
    class={css({
        whiteSpace: "pre-wrap",
        lineHeight: "relaxed",
        color: "bone.100",
    })}
>
    {log.body}
</article>
