<script lang="ts">
    import LogDetail from "$lib/components/LogDetail.svelte";
    import { RATING_CLASSES, STATUS_CLASSES, STATUS_LABELS } from "$lib/logDisplay";
    import { isValidLog, STATUSES } from "$lib/log";
    import { logStore } from "$lib/logStore.svelte";
    import { Select } from "bits-ui";
    import { css } from "styled-system/css";
    import { flex, grid } from "styled-system/patterns";

    /** Logs that failed at any stage, in file name order. */
    const failed = $derived(logStore.logs.filter((log) => !isValidLog(log)));

    /** Valid logs sorted alphabetically by display title. */
    const games = $derived(
        logStore.logs.filter(isValidLog).toSorted((a, b) =>
            a.metadata.title.localeCompare(b.metadata.title, undefined, {
                sensitivity: "base",
            }),
        ),
    );

    const filterItems = [
        { value: "all", label: "All" },
        ...STATUSES.map((status) => ({
            value: status,
            label: STATUS_LABELS[status],
        })),
    ];
    let filter = $state("all");

    /** Sorted logs narrowed to the selected status. */
    const visible = $derived(
        filter === "all"
            ? games
            : games.filter((log) => log.metadata.details.status === filter),
    );

    let selectedFileName = $state<string | null>(null);
    let showFailed = $state(false);
    const selected = $derived(games.find((log) => log.fileName === selectedFileName));

    const cell = css({ px: "2", py: "0.5", textAlign: "left" });
</script>

<!-- NOTE: two-row grid keeps the table and detail borders aligned under the toolbar -->
<div
    class={grid({
        h: "full",
        columns: 2,
        gridTemplateRows: "auto minmax(0, 1fr)",
        columnGap: "3",
        rowGap: "1.5",
    })}
>
    <!-- Filter bar -->
    <div
        class={flex({
            gridColumn: "1",
            gridRow: "1",
            justify: "space-between",
            align: "center",
        })}
    >
        <div class={flex({ align: "center", gap: "2", fontSize: "xs" })}>
            <span class={css({ color: "bone.300" })}>{visible.length} games</span>
            {#if failed.length}
                <button
                    type="button"
                    onclick={() => (showFailed = true)}
                    class={css({
                        color: "ember.400",
                        cursor: "pointer",
                        _hover: { textDecoration: "underline" },
                    })}
                >
                    {failed.length} failed
                </button>
            {/if}
        </div>

        <Select.Root type="single" bind:value={filter} items={filterItems}>
            <Select.Trigger
                class={flex({
                    align: "center",
                    gap: "2",
                    px: "2",
                    py: "0.5",
                    fontSize: "xs",
                    borderWidth: "1px",
                    borderColor: "ink.700",
                    bg: "ink.800",
                    cursor: "pointer",
                    _hover: { bg: "ink.700" },
                })}
            >
                <Select.Value placeholder="Filter" />
                <span aria-hidden="true">▾</span>
            </Select.Trigger>
            <Select.Portal>
                <Select.Content
                    sideOffset={4}
                    align="end"
                    class={css({
                        minW: "36",
                        p: "1",
                        borderWidth: "1px",
                        borderColor: "ink.700",
                        bg: "ink.800",
                        color: "bone.50",
                        shadow: "lg",
                        zIndex: 50,
                    })}
                >
                    {#each filterItems as item (item.value)}
                        <Select.Item
                            value={item.value}
                            label={item.label}
                            class={css({
                                px: "2",
                                py: "1",
                                fontSize: "xs",
                                cursor: "pointer",
                                _highlighted: { bg: "ink.700" },
                                _selected: { color: "arcane.300" },
                            })}
                        >
                            {item.label}
                        </Select.Item>
                    {/each}
                </Select.Content>
            </Select.Portal>
        </Select.Root>
    </div>

    <!-- Games table -->
    <div
        class={css({
            gridColumn: "1",
            gridRow: "2",
            overflow: "auto",
            borderWidth: "1px",
            borderColor: "ink.700",
        })}
    >
        <table class={css({ w: "full", fontSize: "xs" })}>
            <thead
                class={css({
                    position: "sticky",
                    top: "0",
                    bg: "ink.800",
                    color: "bone.300",
                    fontSize: "xs",
                })}
            >
                <tr>
                    <th class={cell}>Title</th>
                    <th class={cell}>Status</th>
                    <th class={cell}>Rating</th>
                </tr>
            </thead>
            <tbody>
                {#each visible as log (log.fileName)}
                    <tr
                        onclick={() => {
                            selectedFileName = log.fileName;
                            showFailed = false;
                        }}
                        aria-current={log.fileName === selectedFileName ? "true" : undefined}
                        class={css({
                            cursor: "pointer",
                            borderTopWidth: "1px",
                            borderColor: "ink.800",
                            // NOTE: Having hover skip current row
                            "&:not([aria-current=true]):hover": { bg: "ink.800" },
                            "&[aria-current=true]": { bg: "arcane.800" },
                        })}
                    >
                        <td class={cell}>
                            <!-- NOTE: button makes rows keyboard-selectable -->
                            <button
                                type="button"
                                class={css({
                                    textAlign: "left",
                                    cursor: "pointer",
                                    _focusVisible: { outline: "1px solid", outlineColor: "arcane.300" },
                                })}
                            >
                                {log.metadata.title}
                            </button>
                        </td>
                        <td class={cell}>
                            {#if log.metadata.details.status}
                                <span class={STATUS_CLASSES[log.metadata.details.status]}
                                    >{STATUS_LABELS[log.metadata.details.status]}</span
                                >
                            {/if}
                        </td>
                        <td class={cell}>
                            {#if log.metadata.details.rating}
                                <span class={RATING_CLASSES[log.metadata.details.rating]}
                                    >{log.metadata.details.rating}</span
                                >
                            {/if}
                        </td>
                    </tr>
                {/each}
            </tbody>
        </table>
    </div>

    <!-- Selected log -->
    <section
        class={css({
            gridColumn: "2",
            gridRow: "2",
            overflow: "auto",
            p: "3",
            borderWidth: "1px",
            borderColor: "ink.700",
        })}
    >
        {#if showFailed}
            <!-- TODO(agent): placeholder until the failed-log notification and editor exist -->
            <h1 class={css({ fontSize: "xl", fontWeight: "bold", mb: "2" })}>
                Failed logs
            </h1>
            {#each failed as log (log.fileName)}
                <section class={css({ mb: "4" })}>
                    <code class={css({ fontWeight: "bold" })}>{log.fileName}</code>
                    <pre
                        class={css({
                            mt: "1",
                            fontSize: "xs",
                            whiteSpace: "pre-wrap",
                            color: "ember.100",
                        })}>{log.error?.message}</pre>
                </section>
            {/each}
        {:else if selected}
            <LogDetail log={selected} />
        {:else}
            <p class={css({ color: "bone.300" })}>Select a game to see its log.</p>
        {/if}
    </section>
</div>
