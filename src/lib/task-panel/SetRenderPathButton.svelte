<script lang="ts">
    import { Command } from "../commands";
    const onclick = Command.choose_render_directory;
</script>

<button {onclick}>Choose...</button>

<style lang="scss">
    button {
        margin-left: 0.8rem;
    }
</style>
