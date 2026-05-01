<script lang="ts">
    import type { Channel } from "@tauri-apps/api/core";
    import type { RenderProgress } from "./progress";
    import { Command } from "../../../commands";

    type Props = {
        progressChannel: Channel<RenderProgress>;
    };
    let { progressChannel }: Props = $props();

    let paused = $state(false);
    const onclick = () => {
        if (paused) {
            Command.resume_task(progressChannel.id);
            paused = false;
        } else {
            Command.pause_task(progressChannel.id);
            paused = true;
        }
    };
</script>

<button {onclick}>{paused ? "Resume" : "Pause"}</button>
