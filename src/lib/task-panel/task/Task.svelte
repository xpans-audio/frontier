<script lang="ts">
    import type { RenderTask } from "../task";
    import RemoveButton from "./RemoveButton.svelte";
    import RenderButton from "./RenderButton.svelte";
    import TaskConfiguration from "./Configuration.svelte";
    import Status from "./status/Status.svelte";

    type Props = {
        taskList: RenderTask[];
        index: number;
        renderDisabled: boolean;
    };
    let { taskList = $bindable(), index, renderDisabled }: Props = $props();
    let task = $derived(taskList[index]);
</script>

<div class="whole">
    <TaskConfiguration bind:task></TaskConfiguration>
    <Status bind:progressChannel={task.progressChannel} />
    <div class="bottom">
        <RemoveButton {taskList} {index} />
        <RenderButton {task} disabled={renderDisabled} />
    </div>
</div>

<style lang="scss">
    @use "../../styles/xpans";
    .whole {
        background-color: xpans.$extra-dark-purple;
        border-radius: 0.8rem;
        padding: 0.8rem;
    }
    .bottom {
        display: flex;
        justify-content: space-between;
    }
</style>
