<script lang="ts">
    import "../app.css";
    import { page } from "$app/state";
    import { css } from "styled-system/css";
    import { flex } from "styled-system/patterns";

    let { children } = $props();

    const navItems = [{ href: "/games", label: "Games" }];
</script>

<div class={flex({ h: "screen", bg: "ink.900", color: "bone.50" })}>
    <nav
        class={flex({
            direction: "column",
            gap: "1",
            w: "56",
            flexShrink: "0",
            p: "4",
            bg: "ink.800",
            borderRightWidth: "1px",
            borderColor: "ink.700",
        })}
    >
        <span
            class={css({
                fontSize: "lg",
                fontWeight: "bold",
                px: "3",
                pb: "4",
            })}
        >
            Grimoire
        </span>
        {#each navItems as item (item.href)}
            <a
                href={item.href}
                aria-current={page.url.pathname.startsWith(item.href)
                    ? "page"
                    : undefined}
                class={css({
                    px: "3",
                    py: "2",
                    rounded: "md",
                    color: "bone.200",
                    _hover: { bg: "ink.700", color: "bone.50" },
                    _currentPage: { bg: "arcane.800", color: "bone.50" },
                })}
            >
                {item.label}
            </a>
        {/each}
    </nav>

    <main class={css({ flex: "1", overflow: "auto", p: "8" })}>
        {@render children()}
    </main>
</div>
