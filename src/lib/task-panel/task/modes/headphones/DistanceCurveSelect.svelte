<script lang="ts">
    import type { DistanceCurve } from "../../../config_mode";

    type Props = {
        selected: DistanceCurve;
    };
    let { selected = $bindable() }: Props = $props();

    type DistanceCurveOption = { name: string; value: DistanceCurve };
    const options: DistanceCurveOption[] = [
        { name: "Sine", value: "sine" },
        { name: "Square Root", value: "square_root" },
        { name: "Linear", value: "linear" },
        { name: "Exponential", value: "exponential" },
    ];
</script>

<div class="config-row">
    <p class="config-label">Distance Curve</p>
    <select bind:value={selected}>
        {#each options as option}
            <option value={option.value}>{option.name}</option>
        {/each}
    </select>
</div>
