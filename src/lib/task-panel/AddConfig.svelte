<script lang="ts">
    import { new_headphones, new_mono, new_stereo } from "./config_mode";

    import { new_task, type RenderTask } from "./task";

    let { taskList = $bindable<RenderTask[]>() } = $props();
    let options = [
        { name: "Mono", default: new_mono },
        { name: "Stereo", default: new_stereo },
        { name: "Headphones", default: new_headphones },
    ];

    let select;

    const onchange = () => {
        let index = select.value;
        let config_mode = options[index - 1].default();
        let task = new_task(config_mode);
        taskList.push(task);

        select.value = 0;
    };
</script>

<select bind:this={select} {onchange}>
    <option value="0" disabled selected>Add task</option>
    {#each options as option, index}
        <option value={index + 1}>{option.name}</option>
    {/each}
</select>
