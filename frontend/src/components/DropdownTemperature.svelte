<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
</script>

<label class="form-control w-full max-w-xs">
  <div class="label">
    <span class="label-text">{t("typeO.dropdownLabel")}</span>
  </div>
  <select class="select select-bordered">
    <option disabled selected hidden>{t("dropdown.firstOption")}</option>
    <option>{t("typeO.dropdownOption1")}</option>
    <option>{t("typeO.dropdownOption2")}</option>
    <option>{t("typeO.dropdownOption3")}</option>
    <option>{t("typeO.dropdownOption4")}</option>
    <option>{t("typeO.dropdownOption5")}</option>
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
