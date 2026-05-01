<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { Events } from "../events";
    import { emptyProject, type Project } from "../project";
    import CreateAndOpenButton from "./CreateAndOpenButton.svelte";
    import CreateOnlyButton from "./CreateOnlyButton.svelte";
    import SpatialPath from "./SpatialPath.svelte";
    import AudioPath from "./AudioPath.svelte";
    import Popup from "../Popup.svelte";
    import { Command } from "../commands";
    let { active = $bindable<boolean>() } = $props();
    var willLoad = $state<boolean>(false);
    var project = $state<Project>({
        title: "",
        audio: "",
        spatial: "",
        names: {},
    });
    const close = () => {
        active = false;
    };
    const reset = () => {
        project = emptyProject;
    };
    listen(Events.project_loaded, (_) => {
        reset();
        close();
    });
    listen(Events.project_created, (_) => {
        reset();
        close();
    });
    listen<string>(Events.scene_project_file_chosen, (event) => {
      Command.create_project(project, event.payload)
    });
    listen<string>(Events.project_created, (event) => {
      if (willLoad) {
        Command.load_project_from_file(event.payload)
        willLoad = false;
      }
    });
</script>

<Popup bind:active overlayDismisses={true}>
    <div class="content {active ? "active" : ""}">
        <h2>New project</h2>
        <div class="inputs">
            <input
                class="title"
                type="text"
                placeholder="Title..."
                bind:value={project.title}
                required
            />
            <AudioPath bind:audioPath={project.audio} />
            <SpatialPath bind:spatialPath={project.spatial} />
        </div>
        <div class="buttons">
            <button onclick={close}>Cancel</button>
            <div class="buttons-right">
                <CreateOnlyButton {project} bind:willLoad/>
                <CreateAndOpenButton {project} bind:willLoad/>
            </div>
        </div>
    </div>
</Popup>

<style lang="scss">
    @use "../styles/xpans";
    .buttons {
        margin-top: 0.8rem;
        display: flex;
        justify-content: space-between;
    }
    .buttons-right {
        display: flex;
        gap: 0.8rem;
    }
    .inputs {
        display: flex;
        flex-direction: column;
    }
    .title {
        width: auto;
        margin-top: 0;
        margin-bottom: 0.8rem;
    }
    h2 {
        font-size: 1.5rem;
        margin-top: 0;
        margin-bottom: xpans.compensated-length(1.6rem, 1.5rem);
    }
    .content {
        background-color: xpans.$dark-purple;
        padding: 1.6rem;
        padding-top: xpans.compensated-length(1.6rem, 1.5rem);
        pointer-events: auto;
        opacity: 0;
        z-index: 11;
        margin: auto;
        width: 32rem;
        border-radius: 0.8rem;
        transform: scale(0.9);
        filter: blur(0.8rem);
        transition:
            transform 250ms ease-in-out,
            opacity 250ms ease-in-out,
            filter 250ms ease-in-out;
    }
    .content.active {
        opacity: 1;
        transform: scale(1);
        filter: blur(0);
    }

</style>
