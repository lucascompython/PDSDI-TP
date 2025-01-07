<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
</script>

<div class="stats shadow div-margin">
  <div class="stat">
    <div class="stat-title">{t("stats.total.outfits.created")}</div>
    <div class="stat-value">89,400</div>
    <div class="stat-desc">{t("stats.and_couting")}!</div>
  </div>
</div>

<style>
  .div-margin {
    margin-top: 1rem;
  }
  div {
    background-color: var(--text-color);
    color: var(--bg-color);
  }
</style>
