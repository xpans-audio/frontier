<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { Command } from "../commands";
    import { Events } from "../events";
    let { spatialPath = $bindable<string>() } = $props();

    const onclick = () => {
        Command.choose_scene_spatial_file();
    };
    listen<string>(Events.scene_spatial_file_chosen, (event) => {
        spatialPath = event.payload;
    });
</script>

<div>
    <input
        type="text"
        placeholder="Path to spatial file..."
        bind:value={spatialPath}
        required
    />
    <button {onclick}>Choose...</button>
</div>

<style lang="scss">
    div {
        display: flex;
        margin-bottom: 0.8rem;
        gap: 0.8rem;
        justify-content: space-between;
    }
    input {
        width: auto;
        flex-grow: 1;
    }
    button {
        width: auto;
    }
</style>
