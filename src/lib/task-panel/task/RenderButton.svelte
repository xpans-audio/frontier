<script lang="ts">
    import { Command } from "../../commands";
    import type { RenderTask } from "../task";

    type Props = {
        task: RenderTask;
        disabled: boolean;
    };
    let { task = $bindable(), disabled }: Props = $props();

    const onclick = async () => {
        Command.render_task(task);
    };
</script>

<button {onclick} {disabled}>Render</button>
