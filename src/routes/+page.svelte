<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import { Button } from "bits-ui";
    import { css } from "styled-system/css";

    let response = $state("");

    /** Calls the backend `ping` command and stores its reply. */
    async function ping() {
        response = await invoke<string>("ping");
    }
</script>

<h1 class={css({ fontSize: "2xl", fontWeight: "bold" })}>Grimoire</h1>

<Button.Root
    class={css({
        px: "4",
        py: "2",
        bg: "arcane.600",
        color: "bone.50",
        rounded: "md",
        cursor: "pointer",
    })}
    onclick={ping}>Ping</Button.Root
>
<p>{response}</p>
