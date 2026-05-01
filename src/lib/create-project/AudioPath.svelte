<script lang="ts">
    import { listen } from "@tauri-apps/api/event";
    import { Command } from "../commands";
    import { Events } from "../events";
    type Props = {
        audioPath: String | null;
    };
    let { audioPath = $bindable() }: Props = $props();

    const onclick = () => {
        Command.choose_scene_audio_file();
    };

    listen<String>(Events.scene_audio_file_chosen, (event) => {
        audioPath = event.payload;
    });
</script>

<div>
    <input
        type="text"
        placeholder="Path to audio file..."
        bind:value={audioPath}
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
