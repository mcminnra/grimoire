<script lang="ts">
    import { isValidLog, type Rating } from "$lib/log";
    import { logStore } from "$lib/logStore.svelte";
    import { RATING_CLASSES, STATUS_CLASSES, STATUS_LABELS } from "$lib/logDisplay";
    import { averageRatingByTag, statusCounts } from "$lib/stats";
    import { css, cx } from "styled-system/css";
    import { grid } from "styled-system/patterns";

    const logs = $derived(logStore.logs.filter(isValidLog));
    const statuses = $derived(statusCounts(logs));
    const tagRatings = $derived(averageRatingByTag(logs));

    const panel = css({
        borderWidth: "1px",
        borderColor: "ink.700",
        overflow: "auto",
    });
    const heading = css({
        px: "2",
        py: "1",
        fontSize: "xs",
        fontWeight: "bold",
        color: "bone.300",
        bg: "ink.800",
        borderBottomWidth: "1px",
        borderColor: "ink.700",
    });
    const cell = css({ px: "2", py: "0.5", textAlign: "left" });
    const num = css({ px: "2", py: "0.5", textAlign: "right", fontVariantNumeric: "tabular-nums" });
</script>

<div
    class={grid({
        columns: 2,
        gap: "3",
        h: "full",
        alignItems: "start",
        fontSize: "xs",
    })}
>
    <!-- Status counts -->
    <section class={panel}>
        <h2 class={heading}>Status</h2>
        <table class={css({ w: "full" })}>
            <tbody>
                {#each statuses as { status, count } (status ?? "none")}
                    <tr class={css({ borderTopWidth: "1px", borderColor: "ink.800" })}>
                        <td class={cell}>
                            {#if status}
                                <span class={STATUS_CLASSES[status]}>{STATUS_LABELS[status]}</span>
                            {:else}
                                <span class={css({ color: "bone.300" })}>No status</span>
                            {/if}
                        </td>
                        <td class={num}>{count}</td>
                    </tr>
                {/each}
                <tr class={css({ borderTopWidth: "1px", borderColor: "ink.700", fontWeight: "bold" })}>
                    <td class={cell}>Total</td>
                    <td class={num}>{logs.length}</td>
                </tr>
            </tbody>
        </table>
    </section>

    <!-- Average rating by tag -->
    <section class={cx(panel, css({ maxH: "full" }))}>
        <h2 class={heading}>Average rating by tag</h2>
        {#if tagRatings.length}
            <table class={css({ w: "full" })}>
                <thead class={css({ color: "bone.300" })}>
                    <tr>
                        <th class={cell}>Tag</th>
                        <th class={num}>Avg</th>
                        <th class={num}>Rated</th>
                    </tr>
                </thead>
                <tbody>
                    {#each tagRatings as { tag, average, count } (tag)}
                        <tr class={css({ borderTopWidth: "1px", borderColor: "ink.800" })}>
                            <td class={cell}>{tag}</td>
                            <td class={num}>
                                <span class={RATING_CLASSES[Math.round(average) as Rating]}
                                    >{average.toFixed(2)}</span
                                >
                            </td>
                            <td class={num}>{count}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        {:else}
            <p class={css({ p: "2", color: "bone.300" })}>No rated logs with tags yet.</p>
        {/if}
    </section>
</div>
