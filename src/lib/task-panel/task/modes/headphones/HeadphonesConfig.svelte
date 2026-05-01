<script lang="ts">
    import DistanceCurveSelect from "./DistanceCurveSelect.svelte";
    import PanLawSelect from "../PanLawSelect.svelte";
    import type { HeadphonesConfiguration } from "../../../config_mode";

    type Props = {
        config: HeadphonesConfiguration;
    };
    let { config = $bindable() }: Props = $props();
</script>

<PanLawSelect bind:selected={config.pan_law} />

<div class="config-row">
    <p class="config-label">Max ITD (ns)</p>
    <div class="slider-combo">
        <input
            type="range"
            min="0"
            max="1000000"
            bind:value={config.max_itd_nanos}
        />
        <input
            type="number"
            min="0"
            max="1000000"
            bind:value={config.max_itd_nanos}
        />
    </div>
</div>

<DistanceCurveSelect bind:selected={config.distance_curve} />

<div class="config-row">
    <p class="config-label">Distance effect</p>
    <div class="slider-combo">
        <input
            type="range"
            min="0"
            max="1"
            step="0.00001"
            bind:value={config.distance_effect}
        />

        <input
            type="number"
            min="0"
            max="1"
            step="0.01"
            bind:value={config.distance_effect}
        />
    </div>
</div>

<div class="config-row">
    <p class="config-label">Minimum Distance</p>
    <div class="slider-combo">
        <input
            type="range"
            min="0"
            max="1"
            step="0.00001"
            bind:value={config.min_distance}
        />

        <input
            type="number"
            min="0"
            step="0.01"
            bind:value={config.min_distance}
        />
    </div>
</div>

<div class="config-row">
    <p class="config-label">Maximum Distance</p>
    <div class="slider-combo">
        <input
            type="range"
            min="0"
            max="2"
            step="0.00001"
            bind:value={config.max_distance}
        />

        <input
            type="number"
            min="0"
            step="0.01"
            bind:value={config.max_distance}
        />
    </div>
</div>
