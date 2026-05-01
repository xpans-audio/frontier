<script lang="ts">
    import { render_many } from "../commands";
    import type { RenderTask } from "./task";

    type Props = {
        taskList: RenderTask[];
        renderDisabled: boolean;
    };
    let { taskList = $bindable(), renderDisabled }: Props = $props();

    const onclick = async () => render_many(taskList);
</script>

<button {onclick} disabled={renderDisabled}>Render All</button>

<style lang="scss">
</style>
