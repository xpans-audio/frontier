<script lang="ts">
    import { Command } from "./commands";
    import Popup from "./Popup.svelte";
    let active = $state(false);
    Command.should_show_dev_warning().then((should) => {
      active = should;
    });
    const onclick = () => {
      Command.acknowledge_dev_warning();
      active = false;
    };
</script>
<Popup bind:active overlayDismisses={false}>
    <div class="content {active ? "active" : ""}">
        <h2>Warning</h2>
        <p><strong>Frontier is very early in development.</strong></p>
        <p>You will likely encounter bugs, missing features, poor performance, and general instability.</p>
        <div class="buttons">
            <button {onclick}>Okay</button>
        </div>
    </div>
</Popup>

<style lang="scss">
    @use "styles/xpans";
    .buttons {
        display: flex;
        justify-content: right;
    }
    h2 {
        font-size: 1.5rem;
        margin-top: 0;
        margin-bottom: xpans.compensated-length(1.6rem, 1.5rem);
    }
    p {
        margin-top: 0;
        margin-bottom: xpans.compensated-length(1.6rem, 1rem);
    }
    .content {
        background-color: xpans.$dark-purple;
        padding: 1.6rem;
        padding-top: xpans.compensated-length(1.6rem, 1.5rem);
        pointer-events: auto;
        opacity: 0;
        z-index: 11;
        margin: auto;
        width: 32rem;
        border-radius: 0.8rem;
        transform: scale(0.9);
        filter: blur(0.8rem);
        transition:
            transform 250ms ease-in-out,
            opacity 250ms ease-in-out,
            filter 250ms ease-in-out;
    }
    .content.active {
        opacity: 1;
        transform: scale(1);
        filter: blur(0);
    }
</style>
