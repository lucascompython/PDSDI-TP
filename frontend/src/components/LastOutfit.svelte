<script lang="ts">
  import { onMount } from "svelte";
  import EyeCard from "./EyeCard.svelte";
  import { type ClotheResponse, getLastOutfit } from "../api/utils";
  import { getLangFromUrl, useTranslations } from "../i18n/utils";

  const { windowLocation }: { windowLocation: URL } = $props();
  const t = useTranslations(getLangFromUrl(windowLocation));

  let clothes: ClotheResponse[] = $state([]);

  onMount(() => {
    getLastOutfit().then((response) => {
      clothes = response;
    });
  });
</script>

{#each clothes as clothe}
  <EyeCard {clothe} {t} />
{/each}

{#if clothes.length === 0}
  <div class="container">
    <div class="text-center text-lg flex">
      {t("clothes.no_outfits")}...
    </div>
  </div>
{/if}
