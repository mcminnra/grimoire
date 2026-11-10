<script lang="ts">
    import "../app.css";
    import { center, flex } from "styled-system/patterns";

    let { children } = $props();
</script>

<main
    class={flex({
        direction: "column",
        h: "screen",
        align: "center",
        bg: "ink.900",
        p: "8",
        color: "bone.50",
    })}
>
    {@render children()}
</main>
