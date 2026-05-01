<script lang="ts">
    let { active = $bindable<boolean>() } = $props();
    const onclick = () => {
        active = true;
    };
</script>

<button {onclick}>New Project</button>
