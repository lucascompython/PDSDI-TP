<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  import { OutfitType } from "src/api/bool_pack";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
</script>

<label class="form-control w-full max-w-xs">
  <div class="label">
    <span class="label-text ml-1">{t("outfit.dropdown.label")}</span>
  </div>
  <select id="outfit-type-select" class="select select-bordered ml-1">
    <option value={null} disabled selected hidden
      >{t("outfit.dropdown.pick_one")}</option
    >
    {#each Object.keys(OutfitType).filter((key: string) => !isNaN(Number(OutfitType[key as keyof typeof OutfitType]))) as type}
      <option value={type}>{t(type as any)}</option>
    {/each}
  </select>
</label>

<style>
  option {
    color: var(--text-color);
  }
  span {
    color: var(--text-color);
  }
  select {
    background-color: var(--bg-color);
    color: var(--text-color);
  }
</style>
