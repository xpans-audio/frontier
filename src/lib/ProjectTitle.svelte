<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { Events } from "./events";
    import type { Project } from "./project";
    let title: String | null = $state(null);
    listen<Project>(Events.project_loaded, (event) => {
        title = event.payload.title;
    });
</script>

{#if (title !== null)}
<p class="project-title">{title}</p>
{/if}
