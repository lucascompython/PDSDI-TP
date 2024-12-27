<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  const lang = getLangFromUrl(window.location.pathname);
  const t = useTranslations(lang);
</script>

<div class="stats shadow div-margin">
  <div class="stat">
    <div class="stat-title">{t("kek.total.outfits.created")}</div>
    <div class="stat-value">89,400</div>
    <div class="stat-desc">21% more than last month</div>
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
