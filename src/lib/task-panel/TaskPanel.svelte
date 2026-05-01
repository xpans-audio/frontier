<script lang="ts">
    import AddConfig from "./AddConfig.svelte";
    import RenderAllButton from "./RenderAllButton.svelte";
    import RenderList from "./RenderList.svelte";
    import type { RenderTask } from "./task";
    import ExportQueue from "./ExportQueue.svelte";
    import ImportQueue from "./ImportQueue.svelte";
    import RenderPath from "./RenderPath.svelte";
    import SetRenderPathButton from "./SetRenderPathButton.svelte";
    import { listen } from "@tauri-apps/api/event";
    import { Events } from "../events";
    import { Command } from "../commands";

    let taskList: RenderTask[] = $state([]);

    let project_loaded: boolean = $state(false);

    let renderDirectory: string = $state("");
    $effect(() => {
        Command.set_render_directory(renderDirectory);
    });
    listen<string>(Events.render_dir_chosen, (event) => {
        renderDirectory = event.payload;
    });
    listen<string>(Events.project_loaded_from_path, async (event) => {
        if (renderDirectory == "") {
            renderDirectory = await Command.get_default_render_dir(
                event.payload,
            );
        }
    });

    let renderDisabled = $derived(
        taskList.length == 0 || !project_loaded || renderDirectory.length == 0,
    );
    listen(Events.project_unloaded, (_) => {
        project_loaded = false;
    });
    listen(Events.project_loaded, (_) => {
        project_loaded = true;
    });
</script>

<div class="panel">
    <div class="panel-header">
        <h2>Render Tasks</h2>
        <AddConfig bind:taskList />
    </div>

    <div class="task-list">
        <RenderList bind:taskList {renderDisabled} />
    </div>
    <div class="output-dir">
        <RenderPath bind:renderDirectory />
        <SetRenderPathButton />
    </div>
    <div class="panel-bottom-bar">
        <ImportQueue bind:queue={taskList} />
        <ExportQueue queue={taskList} />
        <RenderAllButton {taskList} {renderDisabled}></RenderAllButton>
    </div>
</div>

<style lang="scss">
    @use "../styles/xpans";
    h2 {
        margin: auto 0;
        font-size: 1rem;
    }
    .panel-header {
        display: flex;
        justify-content: space-between;
    }
    .task-list {
        overflow-y: scroll;
        flex-grow: 1;
    }
    .output-dir {
        display: flex;
    }
    .panel {
        box-sizing: border-box;
        display: flex;
        flex-direction: column;
        position: absolute;
        height: 100vh;
        min-width: 24rem + 1.6rem;
        right: 0;
        top: 0;
        padding: 0.8rem;
        gap: 0.8rem;
        background-color: xpans.$dark-purple;
    }
    .panel-bottom-bar {
        display: flex;
        justify-content: right;
    }
</style>
