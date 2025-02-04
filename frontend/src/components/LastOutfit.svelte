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
