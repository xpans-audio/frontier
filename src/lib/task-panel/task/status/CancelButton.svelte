<script lang="ts">
    import type { Channel } from "@tauri-apps/api/core";
    import type { RenderProgress } from "./progress";
    import { Command } from "../../../commands";

    type Props = {
        progressChannel: Channel<RenderProgress>;
    };
    let { progressChannel }: Props = $props();

    let confirming: boolean = $state(false);
    const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

    const onclick = async () => {
        if (confirming) {
            cancel();
        } else {
            confirm();
        }
    };
    const confirm = async () => {
        confirming = true;
        await delay(3000);
        confirming = false;
    };
    const cancel = () => {
        Command.cancel_task(progressChannel.id);
    };
</script>

<button class={confirming ? "confirming" : ""} {onclick}>
    {#if confirming}
        Confirm
    {:else}
        Cancel
    {/if}
</button>

<style lang="scss">
    @use "../../../styles/xpans";
    button {
        transition: background-color 250ms ease-in-out;
    }
    .confirming {
        background-color: xpans.$accent-red;
    }
</style>
