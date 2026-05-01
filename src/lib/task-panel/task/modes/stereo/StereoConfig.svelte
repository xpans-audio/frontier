<script lang="ts">
    import PanLawSelect from "../PanLawSelect.svelte";
    import StereoModeSelect from "./StereoModeSelect.svelte";
    import type { StereoConfiguration } from "../../../config_mode";

    type Props = {
        config: StereoConfiguration;
    };
    let { config = $bindable() }: Props = $props();
</script>

<StereoModeSelect bind:selected={config.mode} />
<PanLawSelect bind:selected={config.pan_law} />
