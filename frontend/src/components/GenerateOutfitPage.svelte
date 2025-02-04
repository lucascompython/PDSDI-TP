<script lang="ts">
  import { getLangFromUrl, useTranslations } from "../i18n/utils";

  import DropdownTemperature from "./DropdownTemperature.svelte";
  import ColorsCheckbox from "./ColorsCheckbox.svelte";
  import EyeCard from "./EyeCard.svelte";
  import { clothes, loading } from "./stores";

  const { windowLocation }: { windowLocation: URL } = $props();
  const t = useTranslations(getLangFromUrl(windowLocation));
</script>

<div class="mt-4"></div>

{#if window.innerWidth < 940}
  <div class="split-screen">
    <div>
      <DropdownTemperature {windowLocation} />
      <div class="margin-top">
        <ColorsCheckbox {windowLocation} />
      </div>

      <div class="divider"></div>

      <div class="centers">
        {#if $loading}
          <div class="loading-container">
            <span class="loading loading-ring loading-xl"></span>
          </div>
        {/if}
        <div>
          {#each $clothes as clothe}
            <div class="margin-top">
              <EyeCard {clothe} {t} />
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <style>
    .loading-container {
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100%;
    }
    .loading {
      height: 100px;
      width: 100px;
    }

    main {
      color: white;
      display: flex;
      flex-direction: column;
      flex: 1;
      background-color: #d9d9d9;
    }
    .split-screen {
      flex: 1;
      display: flex;
      width: 100%;

      justify-content: center;
    }
    :global(.dark) main {
      background-color: #0d1117;
    }
    .margin-top {
      margin-top: 1rem;
    }
    .centers {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
    }
  </style>
{:else}
  <div class="split-screen">
    <div class="left">
      <div class="margin-left">
        <DropdownTemperature {windowLocation} />
        <div class="margin-top">
          <ColorsCheckbox {windowLocation} />
        </div>
      </div>
      <div class="line-horizontal"></div>
    </div>
    <div class="right">
      {#if $loading}
        <div class="loading-container">
          <span class="loading loading-ring loading-xl"></span>
        </div>
      {/if}
      <div class="grid-container">
        {#each $clothes as clothe}
          <EyeCard {clothe} {t} />
        {/each}
      </div>
    </div>
    <div class="line-horizontal"></div>
  </div>

  <style>
    .loading-container {
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100%;
    }
    .loading {
      height: 100px;
      width: 100px;
    }

    main {
      color: white;
      display: flex;
      flex-direction: column;
      flex: 1;
      background-color: #d9d9d9;
    }
    .split-screen {
      flex: 1;
      display: flex;
      width: 100%;
      position: relative;
    }
    .left {
      flex-basis: 50%;
      display: flex;
      flex-direction: column;
    }
    :global(.dark) main {
      background-color: #0d1117;
    }
    .right {
      border-left: 3px solid #b3b1b1;
      flex-basis: 50%;
      position: relative;
      max-height: 85vh;
      overflow: auto;
    }
    .margin-left {
      margin-left: 2rem;
    }

    .margin-top {
      margin-top: 1rem;
    }
    .grid-container {
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 1rem;
      width: 100%;
      padding: 1rem;
      justify-items: center;
    }
  </style>
{/if}
