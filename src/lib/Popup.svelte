<script lang="ts">
  import type { Snippet } from 'svelte';
    import PopupOverlay from './PopupOverlay.svelte';
    type Props = {
      children: Snippet,
      overlayDismisses: boolean,
      active: boolean,
    }
    let { children, overlayDismisses, active = $bindable() }: Props = $props();
</script>

<div class="popup-container {active ? 'active' : ''}" inert={!active}>
   	{@render children()}
</div>
<PopupOverlay bind:active dismisses={overlayDismisses} />

<style lang="scss">
    .popup-container {
        pointer-events: none;
        z-index: 10;
        inset: 0;
        position: fixed;
        display: flex;
    }
</style>
