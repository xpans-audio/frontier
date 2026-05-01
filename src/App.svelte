<script lang="ts">
    import LoadProjectButton from "./lib/LoadProjectButton.svelte";
    import ProjectTitle from "./lib/ProjectTitle.svelte";
    import CreateProject from "./lib/create-project/CreateProject.svelte";
    import CreateProjectButton from "./lib/CreateProjectButton.svelte";
    import TaskPanel from "./lib/task-panel/TaskPanel.svelte";
    import DevWarning from "./lib/DevWarning.svelte";
    import { invoke } from "@tauri-apps/api/core";
    invoke("frontend_ready");
    let create_project_active = $state<boolean>(false);
</script>

<main>
    <div class="main-container">
        <div class="home">
            <div class="top-bar">
                <ProjectTitle />
                <CreateProjectButton
                    bind:active={create_project_active}
                />
                <LoadProjectButton />
            </div>
        </div>
        <TaskPanel></TaskPanel>
    </div>
    <CreateProject bind:active={create_project_active}></CreateProject>
    <DevWarning />
</main>

<style lang="scss">
    @use "lib/styles/xpans";
    .main-container {
        position: fixed;
        inset: 0;
        width: 100vw;
        height: 100vh;
        display: flex;
    }
    .home {
        padding: 0.8rem;
    }
    .top-bar {
        display: flex;
        gap: 0.8rem;
    }
    .bottom-bar {
        display: flex;
        justify-content: right;
        background-color: xpans.$dark-purple;
        position: fixed;
        bottom: 0;
        left: 0;
        padding: 0.8rem 0;
        width: 100vw;
    }
</style>
