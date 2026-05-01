<script lang="ts">
    type Props = {
        renderDirectory: string;
    };
    let { renderDirectory = $bindable() }: Props = $props();
</script>

<input
    type="text"
    bind:value={renderDirectory}
    placeholder="Render output directory..."
/>

<style lang="scss">
    input {
        flex-grow: 1;
    }
</style>
