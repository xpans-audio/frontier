<script lang="ts">
    import type { RenderProgress } from "./progress";

    type Props = { renderProgress: RenderProgress | null };
    let { renderProgress }: Props = $props();
</script>

{#if renderProgress?.status == "rendering"}
    <p>Rendering... {Math.floor(renderProgress.progress * 100)}%</p>
    <input
        readonly
        type="range"
        value={renderProgress.progress}
        min="0"
        max="1"
        step="0.001"
    />
{:else if renderProgress?.status == "finished"}<p>Finished!</p>
{:else if renderProgress?.status == "error"}<p>Error!</p>{/if}

<style lang="scss">
    input[type="range"]::-webkit-slider-thumb {
        -webkit-appearance: none;
        appearance: none;
        cursor: pointer;
    }
    input[type="range"]::-moz-range-thumb {
        cursor: pointer;
    }
    input[type="range"] {
        accent-color: cyan;
    }
</style>
