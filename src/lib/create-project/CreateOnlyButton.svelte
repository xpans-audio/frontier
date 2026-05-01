<script lang="ts">
    import { Command } from "../commands";
    import type { Project } from "../project";
    import { notReadyToCreate } from "./create";

    type Props = { project: Project, willLoad: boolean };
    let { project, willLoad = $bindable() }: Props = $props();

    let disabled = $derived(notReadyToCreate(project));

    const onclick = () => {
        willLoad = false;
        Command.choose_scene_project_file();
    };
</script>

<button {onclick} {disabled}>Create only</button>
