<script lang="ts">
  import { onMount } from "svelte";
  import { type ClotheResponse, getClothes } from "../api/utils";
  import EyeCard from "./EyeCard.svelte";
  import {
    getLangFromUrl,
    useTranslations,
    useTranslatedPath,
  } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatedPath = useTranslatedPath(lang);

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

{#if clothes.length === 0 && !loading}
  <div class="container">
    <div class="text-center text-lg flex">
      {t("clothes.no_clothes")}

      <a href={translatedPath("/upload")} class="link link-hover">
        <div class="flex">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            class="mt-1 ml-l mr-1"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            stroke-width="2"
            stroke="currentColor"
            fill="none"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path stroke="none" d="M0 0h24v24H0z" fill="none" />
            <line x1="5" y1="12" x2="19" y2="12" />
            <line x1="13" y1="18" x2="19" y2="12" />
            <line x1="13" y1="6" x2="19" y2="12" />
          </svg>
          {t("clothes.add_clothes")}
        </div>
      </a>
    </div>
  </div>
{/if}

<style>
  .container {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: auto;
    height: 100px;
    color: var(--text-color);
  }

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
