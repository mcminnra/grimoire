<script lang="ts">
    import GameCard from "$lib/components/GameCard.svelte";
    import { Select } from "bits-ui";
    import { css } from "styled-system/css";
    import { flex, grid } from "styled-system/patterns";

    // TODO(agent): replace mock games with real data
    const words = [
        "Lorem",
        "Ipsum",
        "Dolor",
        "Sit",
        "Amet",
        "Consectetur",
        "Adipiscing",
        "Elit",
    ];
    const statuses = ["Playing", "Completed", "Backlog", "Dropped", undefined];

    /** Builds a deterministic mock game for index `i`. */
    function mockGame(i: number) {
        return {
            name: `${words[i % words.length]} ${words[(i * 3 + 1) % words.length]} ${i + 1}`,
            status: statuses[i % statuses.length],
            rating: i % 3 === 0 ? undefined : (i % 10) + 1,
            coverUrl:
                "https://images.igdb.com/igdb/image/upload/t_cover_big/co4jni.webp",
        };
    }

    const games = Array.from({ length: 20 }, (_, i) => mockGame(i));

    // TODO(agent): mock filter, not wired to the grid yet
    const filterItems = [
        { value: "all", label: "All" },
        { value: "playing", label: "Playing" },
        { value: "completed", label: "Completed" },
        { value: "backlog", label: "Backlog" },
        { value: "dropped", label: "Dropped" },
    ];
    let filter = $state("all");
</script>

<!-- Filter bar -->
<div class={flex({ justify: "space-between", align: "center", mb: "4" })}>
    <span class={css({ fontSize: "sm", color: "bone.300" })}
        >{games.length} games</span
    >

    <Select.Root type="single" bind:value={filter} items={filterItems}>
        <Select.Trigger
            class={flex({
                align: "center",
                gap: "2",
                px: "3",
                py: "1.5",
                fontSize: "sm",
                rounded: "md",
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
                    rounded: "md",
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
                            py: "1.5",
                            fontSize: "sm",
                            rounded: "sm",
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

<!-- Games Grid -->
<div
    class={grid({
        columns: { base: 2, sm: 3, md: 4, lg: 5, xl: 6 },
        gap: "4",
    })}
>
    {#each games as game, i (i)}
        <GameCard {game} href="#" />
    {/each}
</div>
