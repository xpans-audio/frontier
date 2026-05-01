<script lang="ts">
    import type { RenderTask } from "./task";
    import Task from "./task/Task.svelte";

    type Props = {
        taskList: RenderTask[];
        renderDisabled: boolean;
    };
    let { taskList = $bindable(), renderDisabled }: Props = $props();
</script>

<div class="render-list">
    {#each taskList, index}
        <Task bind:taskList {index} {renderDisabled} />
    {/each}
</div>

<style lang="scss">
    @use "../styles/xpans";
    .task {
        /* margin: 0.8rem 0; */
        /* border: 1px solid white; */
        background-color: xpans.$extra-dark-purple;
        border-radius: 0.8rem;
        padding: 0.8rem;
    }
    .render-list {
        display: flex;
        flex-direction: column;
        gap: 0.8rem;
    }
</style>
