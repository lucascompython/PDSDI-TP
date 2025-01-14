<script lang="ts">
  import { getLangFromUrl, useTranslations } from "../i18n/utils";

  import DropdownTemperature from "./DropdownTemperature.svelte";
  import ColorsCheckbox from "./ColorsCheckbox.svelte";
  import EyeCard from "@components/EyeCard.svelte";

  const { windowLocation }: { windowLocation: URL } = $props();
  const t = useTranslations(getLangFromUrl(windowLocation));
</script>

{#if window.innerWidth < 600}
  <div class="split-screen">
    <div class="left">
      <DropdownTemperature {windowLocation} />
      <div class="margin-top">
        <ColorsCheckbox {windowLocation} />
      </div>
    </div>
  </div>

  <style>
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
      <div class="eye-cards-column">
        <!-- <EyeCard />
        <EyeCard />
        <EyeCard /> -->
      </div>
    </div>
    <div class="line-horizontal"></div>
  </div>

  <style>
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
    .eye-cards-column {
      display: flex;
      flex-direction: column;
      align-items: center;
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
    }
    .margin-left {
      margin-left: 2rem;
    }

    .margin-top {
      margin-top: 1rem;
    }
  </style>
{/if}
