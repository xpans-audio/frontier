<script lang="ts">
    import type { Channel } from "@tauri-apps/api/core";
    import type { RenderProgress } from "./progress";
    import ProgressBar from "./ProgressBar.svelte";
    import PauseButton from "./PauseButton.svelte";
    import CancelButton from "./CancelButton.svelte";

    type Props = {
        progressChannel: Channel<RenderProgress>;
    };
    let { progressChannel = $bindable() }: Props = $props();

    let renderProgress: RenderProgress | null = $state(null);
    progressChannel.onmessage = (message: RenderProgress) => {
        renderProgress = message;
    };
</script>

{#if renderProgress !== null}
    <div>
        <ProgressBar {renderProgress} />
        {#if renderProgress.status == "rendering"}
            <PauseButton {progressChannel} />
            <CancelButton {progressChannel} />
        {/if}
    </div>
{/if}

<style lang="scss">
    div {
        display: flex;
    }
</style>
