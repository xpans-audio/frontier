<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { Command } from "../commands";
    import { taskForExport, type RenderTask } from "./task";
    import { Events } from "../events";

    let { queue }: { queue: RenderTask[] } = $props();

    let disabled = $derived(queue.length == 0);

    const onclick = () => {
        Command.choose_queue_export();
    };
    listen<string>(Events.queue_export_chosen, (event) => {
        let tasks = queue.map((task) => {
            return taskForExport(task);
        });
        Command.export_queue(tasks, event.payload);
    });
</script>

<button {onclick} {disabled}>Export Queue</button>

<style lang="scss">
    button {
        margin-right: 0.8rem;
    }
</style>
