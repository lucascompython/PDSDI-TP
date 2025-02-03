<script lang="ts">
  import { onMount } from "svelte";
  import { type ClotheResponse, getClothes } from "../api/utils";
  import EyeCard from "./EyeCard.svelte";
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const t = useTranslations(getLangFromUrl(windowLocation));

  let clothes: ClotheResponse[] = $state([]);
  let loading = $state(true);

  onMount(() => {
    getClothes().then((clothesResponse) => {
      loading = false;
      clothes = clothesResponse;
    });
  });
</script>

{#if loading}
  <div class="loading-container">
    <span class="loading loading-ring loading-xl"></span>
  </div>
{/if}

{#each clothes as clothe}
  <EyeCard {clothe} {t} />
{/each}

<style>
  .loading-container {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .loading {
    height: 100px;
    width: 100px;
  }
</style>
