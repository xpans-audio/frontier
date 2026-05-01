<script lang="ts">
    import type { StereoMode } from "../../../config_mode";

    type Props = {
        selected: StereoMode;
    };
    let { selected = $bindable() }: Props = $props();

    type PanLawOption = { name: string; value: StereoMode };
    const options: PanLawOption[] = [
        { name: "Positional", value: "positional" },
        { name: "Directional", value: "directional" },
    ];
</script>

<div class="config-row">
    <p class="config-label">Stereo mode</p>
    <select bind:value={selected}>
        {#each options as option}
            <option value={option.value}>{option.name}</option>
        {/each}
    </select>
</div>
