<script lang="ts">
    type Props = {
        taskList: any[];
        index: number;
    };
    let { taskList = $bindable(), index }: Props = $props();

    let confirming: boolean = $state(false);
    const delay = (ms: number) => new Promise((res) => setTimeout(res, ms));

    const onclick = async () => {
        if (confirming) {
            remove();
        } else {
            confirm();
        }
    };
    const confirm = async () => {
        confirming = true;
        await delay(3000);
        confirming = false;
    };
    const remove = () => {
        taskList.splice(index, 1);
    };
</script>

<button class={confirming ? "confirming" : ""} {onclick}>
    {#if confirming}
        Confirm
    {:else}
        Remove
    {/if}
</button>

<style lang="scss">
    @use "../../styles/xpans";
    button {
        transition: background-color 250ms ease-in-out;
    }
    .confirming {
        background-color: xpans.$accent-red;
    }
</style>
