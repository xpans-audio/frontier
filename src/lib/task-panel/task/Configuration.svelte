<script lang="ts">
    import { ModeNames } from "../config_mode";
    import HeadphonesConfig from "./modes/headphones/HeadphonesConfig.svelte";
    import MonoConfig from "./modes/mono/MonoConfig.svelte";
    import StereoConfig from "./modes/stereo/StereoConfig.svelte";
    import type { RenderTask } from "../task";

    let { task = $bindable<RenderTask>() } = $props();
</script>

<h4>{ModeNames.get(task.renderConfig.mode)}</h4>
<div class="config-row">
    <p class="config-label">Task Name</p>
    <input type="text" class="task-name" bind:value={task.name} />
</div>
{#if task.renderConfig.mode == "headphones"}
    <HeadphonesConfig bind:config={task.renderConfig.config} />
{/if}
{#if task.renderConfig.mode == "stereo"}
    <StereoConfig bind:config={task.renderConfig.config} />
{/if}
{#if task.renderConfig.mode == "mono"}
    <MonoConfig bind:config={task.renderConfig.config} />
{/if}

<style lang="scss">
    .task-name {
        width: 100%;
    }
</style>
