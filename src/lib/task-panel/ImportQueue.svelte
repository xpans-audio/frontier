<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { Command } from "../commands";
    import {
        taskForImport,
        type RenderTask,
        type RenderTaskForExport,
    } from "./task";
    import { Events } from "../events";

    let { queue = $bindable<RenderTask[]>() }: { queue: RenderTask[] } =
        $props();

    const onclick = () => {
        Command.choose_queue_import();
    };
    listen<string>(Events.queue_import_chosen, (event) => {
        Command.import_queue(event.payload);
    });
    listen<RenderTaskForExport[]>(Events.queue_imported, (event) => {
        let tasks = event.payload.map((task) => {
            return taskForImport(task);
        });
        queue = queue.concat(tasks);
    });
</script>

<button {onclick}>Import Queue</button>

<style lang="scss">
    button {
        margin-right: 0.8rem;
    }
</style>
