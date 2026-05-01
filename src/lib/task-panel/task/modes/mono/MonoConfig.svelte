<script lang="ts">
    import type { MonoConfiguration } from "../../../config_mode";

    type Props = {
        config: MonoConfiguration;
    };
    let { config = $bindable() }: Props = $props();
</script>

<div class="config-row">
    <p class="config-label">Channel count</p>
    <input type="number" min="1" bind:value={config.channels} />
</div>
