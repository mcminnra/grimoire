<script lang="ts">
    import "../app.css";
    import { page } from "$app/state";
    import { onLogChanged } from "$lib/log";
    import { logStore } from "$lib/logStore.svelte";
    import { css } from "styled-system/css";
    import { flex } from "styled-system/patterns";

    let { children } = $props();

    // Refetch just the changed log whenever the backend reports one
    $effect(() => {
        const unlisten = onLogChanged((fileName) => logStore.refresh(fileName));
        return () => {
            unlisten.then((stop) => stop());
        };
    });

    const navItems = [
        { href: "/games", label: "Games" },
        { href: "/stats", label: "Stats" },
    ];
</script>

<div
    class={flex({
        direction: "column",
        h: "screen",
        bg: "ink.900",
        color: "bone.50",
    })}
>
    <header
        class={flex({
            align: "center",
            gap: "4",
            h: "8",
            flexShrink: "0",
            px: "3",
            bg: "ink.800",
            borderBottomWidth: "1px",
            borderColor: "ink.700",
            userSelect: "none",
        })}
    >
        <span class={css({ fontSize: "sm", fontWeight: "bold" })}>Grimoire</span>

        <nav class={flex({ align: "center", gap: "1", h: "full" })}>
            {#each navItems as item (item.href)}
                <a
                    href={item.href}
                    aria-current={page.url.pathname.startsWith(item.href)
                        ? "page"
                        : undefined}
                    class={css({
                        px: "2",
                        py: "0.5",
                        fontSize: "xs",
                        color: "bone.200",
                        // NOTE: having hover skip current page
                        "&:not([aria-current=page]):hover": { bg: "ink.700", color: "bone.50" },
                        _currentPage: { bg: "arcane.800", color: "bone.50" },
                    })}
                >
                    {item.label}
                </a>
            {/each}
        </nav>
    </header>

    <main class={css({ flex: "1", minH: "0", overflow: "auto", p: "3" })}>
        {@render children()}
    </main>
</div>
