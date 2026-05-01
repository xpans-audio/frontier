<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { Command } from "../commands";
    import { Events } from "../events";
    import type { Project } from "../project";
    import { notReadyToCreate } from "./create";

    type Props = { project: Project, willLoad: boolean };
    let { project, willLoad = $bindable() }: Props = $props();

    let disabled = $derived(notReadyToCreate(project));

    const onclick = () => {
      willLoad = true;
      Command.choose_scene_project_file();
    };
</script>

<button {onclick} {disabled}>Create and open</button>
