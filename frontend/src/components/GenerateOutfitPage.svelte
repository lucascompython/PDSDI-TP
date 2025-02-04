<script lang="ts">
  import { getLangFromUrl, useTranslations } from "../i18n/utils";

  import DropdownTemperature from "./DropdownTemperature.svelte";
  import ColorsCheckbox from "./ColorsCheckbox.svelte";
  import EyeCard from "./EyeCard.svelte";
  import { clothes, loading } from "./stores";

  const { windowLocation }: { windowLocation: URL } = $props();
  const t = useTranslations(getLangFromUrl(windowLocation));
</script>

{#if window.innerWidth < 940}
  <div class="split-screen">
    <div>
      <DropdownTemperature {windowLocation} />
      <div class="margin-top">
        <ColorsCheckbox {windowLocation} />
      </div>
      <div class="margin-top centers">
        <div class="card bg-base-100 w-64 shadow-xl">
          <figure>
            <img
              src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
              alt="Shoes"
            />
          </figure>
          <div class="card-body">
            <h2 class="card-title">
              Shoes!
              <div class="badge badge-secondary">NEW</div>
            </h2>
            <p>If a dog chews shoes whose shoes does he choose?</p>
            <div class="card-actions justify-end">
              <div class="badge badge-outline">Fashion</div>
              <div class="badge badge-outline">Products</div>
            </div>
          </div>
        </div>
        <div class="card bg-base-100 w-64 shadow-xl">
          <figure>
            <img
              src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"
              alt="Shoes"
            />
          </figure>
          <div class="card-body">
            <h2 class="card-title">
              Shoes!
              <div class="badge badge-secondary">NEW</div>
            </h2>
            <p>If a dog chews shoes whose shoes does he choose?</p>
            <div class="card-actions justify-end">
              <div class="badge badge-outline">Fashion</div>
              <div class="badge badge-outline">Products</div>
            </div>
          </div>
        </div>
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
      <div class="eye-cards-column">
        {#if $loading}
          <div class="loading-container">
            <span class="loading loading-ring loading-xl"></span>
          </div>
        {/if}

        {#each $clothes as clothe}
          <EyeCard {clothe} {t} />
        {/each}
      </div>
    </div>
    <div class="line-horizontal"></div>
  </div>

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
