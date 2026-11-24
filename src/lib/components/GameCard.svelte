<!--
@component
Compact game card with a cover, title, status, and rating. Links to `href`.
Shows a badge when the game's log has parse errors.
-->
<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { css, cx } from "styled-system/css";
    import { flex } from "styled-system/patterns";

    // TODO(agent): replace with the real Game model once it exists
    /** Minimal game shape the card needs to render. */
    interface GameCardGame {
        name: string;
        coverUrl?: string;
        status?: string;
        rating?: number;
        /** Number of parse errors on the backing log. */
        errorCount?: number;
    }

    let {
        game,
        href,
        class: className,
        ...rest
    }: {
        game: GameCardGame;
        href: string;
    } & HTMLAttributes<HTMLDivElement> = $props();

    const metaText = css({ fontSize: "xs" });
</script>

<div
    class={[className, css({ position: "relative" })]}
    {...rest}
    data-testid="game-card"
>
    <a
        {href}
        class={css({
            display: "block",
            borderWidth: "1px",
            borderColor: "ink.700",
            bg: "ink.800",
            transition: "opacity",
            transitionDuration: "100ms",
            transitionTimingFunction: "ease-in-out",
            _hover: { opacity: 0.8 },
        })}
    >
        {#if game.errorCount}
            <span
                title="{game.errorCount} parse error(s)"
                class={css({
                    position: "absolute",
                    top: "1.5",
                    right: "1.5",
                    px: "1.5",
                    fontSize: "xs",
                    fontWeight: "bold",
                    bg: "ember.600",
                    color: "bone.50",
                })}>!{game.errorCount}</span
            >
        {/if}

        <!-- Cover -->
        {#if game.coverUrl}
            <img
                src={game.coverUrl}
                alt="{game.name} cover"
                class={css({
                    w: "full",
                    aspectRatio: "3 / 4",
                    objectFit: "cover",
                })}
            />
        {:else}
            <div
                class={css({
                    w: "full",
                    aspectRatio: "3 / 4",
                    bg: "ink.700",
                })}
            ></div>
        {/if}

        <!-- Title, status, and rating -->
        <div class={flex({ direction: "column", gap: "0.5", p: "2" })}>
            <p
                class={css({
                    fontSize: "xs",
                    fontWeight: "bold",
                    letterSpacing: "wide",
                    color: "bone.100",
                    truncate: true,
                })}
            >
                {game.name}
            </p>
            <div class={flex({ justify: "space-between" })}>
                <span class={cx(metaText, css({ color: "bone.300" }))}
                    >{game.status ?? " "}</span
                >

                {#if game.rating}
                    <div class={flex({ align: "center", gap: "1" })}>
                        <span class={metaText}>{game.rating}</span>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            width={10}
                            height={10}
                            viewBox="0 0 24 24"
                            aria-hidden="true"
                        >
                            <path
                                class={css({ fill: "gold.400" })}
                                d="M12 .587l3.668 7.568 8.332 1.151-6.064 5.828 1.48 8.279-7.416-3.967-7.417 3.967 1.481-8.279-6.064-5.828 8.332-1.151z"
                            />
                        </svg>
                    </div>
                {:else}
                    <span class={metaText}>&nbsp;</span>
                {/if}
            </div>
        </div>
    </a>
</div>
