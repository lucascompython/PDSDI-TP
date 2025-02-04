<script lang="ts">
  import { getLangFromUrl, useTranslations } from "../i18n/utils";

  import DropdownTemperature from "./DropdownTemperature.svelte";
  import ColorsCheckbox from "./ColorsCheckbox.svelte";
  import EyeCard from "./EyeCard.svelte";
  import { clothes, loading } from "./stores";
  import Hanger from "./Icons/Hanger.svg?raw";
  import { onMount } from "svelte";
  import { OutfitType } from "src/api/bool_pack";
  import { showAlert, AlertType } from "./Alert/Alert";
  import { get } from "svelte/store";
  import { saveOutfit } from "src/api/utils";

  const { windowLocation }: { windowLocation: URL } = $props();
  const t = useTranslations(getLangFromUrl(windowLocation));

  let checkboxes: NodeListOf<HTMLInputElement>;
  let outfitTypeSelect: HTMLSelectElement;

  onMount(() => {
    checkboxes = document.querySelectorAll(".checkbox");
    outfitTypeSelect = document.getElementById(
      "outfit-type-select",
    ) as HTMLSelectElement;
  });

  function handleSaveOutfit(e: MouseEvent) {
    e.preventDefault();

    const outfitType = outfitTypeSelect.value as keyof typeof OutfitType;

    const outfitName = (
      document.getElementById("outfitName") as HTMLInputElement
    ).value;

    if (outfitName === "") {
      showAlert(t("outfit.name_required"), AlertType.ERROR);
      return;
    }

    const clothesId = [];
    const clothesValue = get(clothes);
    for (let i = 0; i < clothesValue.length; i++) {
      clothesId.push(clothesValue[i].id);
    }

    saveOutfit(outfitName, OutfitType[outfitType], clothesId).then((ok) => {
      if (!ok) {
        showAlert(t("outfit.save_outfit_error"), AlertType.ERROR);
        return;
      }
      showAlert(t("outfit.save_outfit_success"), AlertType.SUCCESS);
    });
  }

  function handleDiscardOutfit(e: MouseEvent) {
    e.preventDefault();

    for (let i = 0; i < checkboxes.length; i++) {
      checkboxes[i].checked = false;
    }
    outfitTypeSelect.value = "";
    clothes.set([]);
  }
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

        {#if $clothes.length > 0 && !$loading}
          <h3 class="margin-top text-center text-lg underline color">
            {t("outfit.save_outfit")}
          </h3>

          <div class="flex justify-center mt-7">
            <div class="flex flex-col gap-4 w-100">
              <label class="w-100 input input-bordered items-center gap-2">
                {@html Hanger}
                <input
                  id="outfitName"
                  type="text"
                  class="grow w-100"
                  placeholder={t("outfit.name")}
                />
              </label>
              <div class="flex justify-between">
                <button onclick={handleDiscardOutfit} class="btn btn-error">
                  {t("outfit.discard_outfit_button")}
                </button>
                <button onclick={handleSaveOutfit} class="btn btn-success">
                  {t("outfit.save_outfit_button")}
                </button>
              </div>
            </div>
          </div>
        {/if}
      </div>
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
    .color {
      color: var(--text-color);
    }
  </style>
{/if}
