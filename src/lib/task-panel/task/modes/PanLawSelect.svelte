<script lang="ts">
    import type { PanLaw } from "../../config_mode";

    type Props = {
        selected: PanLaw;
    };
    let { selected = $bindable() }: Props = $props();

    type PanLawOption = { name: string; value: PanLaw };
    const options: PanLawOption[] = [
        { name: "Sine (-3 dB)", value: "sine" },
        { name: "Square Root (-3 dB)", value: "square_root" },
        { name: "Natural (-6 dB)", value: "natural" },
    ];
</script>

<div class="config-row">
    <p class="config-label">Pan law</p>
    <select bind:value={selected}>
        {#each options as option}
            <option value={option.value}>{option.name}</option>
        {/each}
    </select>
</div>
