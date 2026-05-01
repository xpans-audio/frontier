<script lang="ts">
    type Props = {
      active: boolean;
      dismisses: boolean;
    }
    let { active = $bindable(), dismisses }: Props = $props();

    const onclick = () => {
      if (dismisses) {
        active = false;
      }
    }
</script>

<div {onclick} class="{dismisses ? "dismisses" : ""}" inert={!active}></div>

<style lang="scss">
    @use "styles/xpans";
    div {
        z-index: 9;
        inset: 0;
        position: fixed;
        background-color: xpans.$extra-dark-purple-translucent;
        backdrop-filter: blur(0.8rem);
        transition:
            background-color 250ms ease-in-out,
            backdrop-filter 250ms ease-in-out;
    }
    div.dismisses {
        cursor: pointer;
    }
    div[inert] {
        background-color: xpans.$extra-dark-purple-transparent;
        backdrop-filter: blur(0rem);
    }
</style>
