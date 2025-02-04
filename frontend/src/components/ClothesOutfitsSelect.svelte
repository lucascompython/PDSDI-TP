<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  import CardLoader from "./CardLoader.svelte";
  import OutfitLoader from "./OutfitLoader.svelte";
  import { onMount } from "svelte";
  const { windowLocation }: { windowLocation: URL } = $props();

  const t = useTranslations(getLangFromUrl(windowLocation));

  let toggle: HTMLInputElement;
  let toggleLabel: HTMLSpanElement;
  let checked = $state(false);

  onMount(() => {
    toggle = document.getElementById("clothe-toggle") as HTMLInputElement;
    toggleLabel = document.getElementById("toggle-label") as HTMLSpanElement;

    toggle.addEventListener("change", () => {
      checked = toggle.checked;
      toggleLabel.textContent = toggle.checked
        ? t("clothes.outfits")
        : t("clothes.clothes");
    });
  });
</script>

{#if !checked}
  <CardLoader {windowLocation} />
{:else}
  <OutfitLoader {windowLocation} />
{/if}
