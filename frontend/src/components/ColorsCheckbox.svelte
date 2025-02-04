<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  import { onMount } from "svelte";
  import { BoolPack, OutfitType } from "src/api/bool_pack";
  import { AlertType, showAlert } from "./Alert/Alert";
  import { generateOutfit, type ClotheResponse } from "src/api/utils";
  import { clothes, loading } from "./stores";

  let { windowLocation }: { windowLocation: URL } = $props();

  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);

  let checkboxes: NodeListOf<HTMLInputElement>;
  let outfitTypeSelect: HTMLSelectElement;

  onMount(() => {
    checkboxes = document.querySelectorAll(".checkbox");
    outfitTypeSelect = document.getElementById(
      "outfit-type-select",
    ) as HTMLSelectElement;
  });

  function handleGenerate(e: MouseEvent) {
    e.preventDefault();

    const boolPack = new BoolPack();

    let isAllFalse = true;
    for (let i = 0; i < checkboxes.length; i++) {
      boolPack.setBool(i, checkboxes[i].checked);
      isAllFalse = isAllFalse && !checkboxes[i].checked;
    }

    if (isAllFalse) {
      showAlert(t("outfit.pick_at_least_one_color"), AlertType.ERROR);
      return;
    }

    const selectValue = outfitTypeSelect.value;
    if (selectValue === "") {
      showAlert(t("outfit.pick_outfit_type"), AlertType.ERROR);
      return;
    }

    const outfitType = selectValue as keyof typeof OutfitType;

    boolPack.setOutfitType(OutfitType[outfitType]);

    loading.set(true);
    generateOutfit(boolPack).then((outfit) => {
      clothes.set(outfit);
      loading.set(false);
    });
  }
</script>

<span class="text-lg ml-1">{t("outfit.type.label")}</span>

<div class="ml-2">
  <div>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-red-500 bg-gray-200 checked:text-black"
      />
      {t("red")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-orange-500 bg-gray-200 checked:text-black"
      />
      {t("orange")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-yellow-500 bg-gray-200 checked:text-black"
      />
      {t("yellow")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-green-500 bg-gray-200 checked:text-black"
      />
      {t("green")}
    </label>
  </div>
  <div>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-blue-500 bg-gray-200 checked:text-black"
      />
      {t("blue")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-purple-500 bg-gray-200 checked:text-black"
      />
      {t("purple")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-pink-500 bg-gray-200 checked:text-black"
      />
      {t("pink")}
    </label>

    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-amber-950 bg-gray-200 checked:text-black"
      />
      {t("brown")}
    </label>
  </div>
  <div>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-black bg-gray-200 checked:text-white"
      />
      {t("black")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-white bg-gray-200 checked:text-black"
      />
      {t("white")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-yellow-400 bg-gray-200 checked:text-black"
      />
      {t("gold")}
    </label>
    <label class="fieldset-label">
      <input
        type="checkbox"
        checked={false}
        class="checkbox checked:bg-gray-500 bg-gray-200 checked:text-black"
      />
      {t("gray")}
    </label>
  </div>
</div>
<div class="center-button">
  <button
    onclick={handleGenerate}
    class="btn btn-xs md:btn-md btn-wide lg:btn-lg"
    >{t("outfit.create_outfit_button")}</button
  >
</div>

<style>
  span {
    color: var(--text-color);
  }
  div {
    display: flex;
    flex-direction: column;
  }
  div div {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    max-width: 97%;
    margin-top: 2rem;
  }
  label {
    align-items: center;
    display: flex;
    min-width: 110px;
    color: var(--text-color);
  }
  button {
    margin-top: 5rem;
    max-width: 200px;
    background-color: var(--text-color);
    color: var(--bg-color);
  }
  button:hover {
    background-color: var(--text-color);
    color: var(--bg-color);
  }
  input {
    border-color: var(--text-color);
    margin-right: 0.5rem;
  }
  .center-button {
    display: flex;
    width: 100%;
    justify-content: center;
    align-items: center;
  }
  @media only screen and (max-width: 940px) {
    div div {
      margin-top: 1rem;
    }
    label {
      min-width: 80px;
      font-size: 0.7rem;
    }
    button {
      margin-top: 2rem;
      min-height: 3rem;
    }
  }
</style>
