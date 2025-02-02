<script lang="ts">
  import { onMount } from "svelte";
  import { type ClotheResponse, getClothes } from "../api/utils";
  import EyeCard from "./EyeCard.svelte";
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const t = useTranslations(getLangFromUrl(windowLocation));

  let clothes: ClotheResponse[] = $state([]);

  onMount(() => {
    getClothes().then((clothesResponse) => {
      clothes = clothesResponse;
    });
  });
</script>

{#each clothes as clothe}
  <EyeCard {clothe} {t} />
{/each}
