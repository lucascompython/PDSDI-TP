<script lang="ts">
  import SettingsIcon from "./Icons/SettingsIcon.svelte";
  import {
    getLangFromUrl,
    useTranslatedPath,
    useTranslations,
  } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatePath = useTranslatedPath(lang);
</script>

<!-- Faltam Merdas -->

<div class="dropdown dropdown-end">
  <div tabindex="0" role="button" class="m-1"><SettingsIcon /></div>
  <ul
    tabindex="-1"
    class="dropdown-content menu bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
  >
    <li><a>Item 1</a></li>
    <li><a>Item 2</a></li>
    <li>
      <button
        class="btn btn-error"
        onclick={() => (window.location.href = translatePath("/login"))}
        >{t("settings.logout")}</button
      >
    </li>
  </ul>
</div>

<style>
  ul {
    background-color: var(--text-color);
    color: var(--bg-color);
  }
  /* BORDA?? */
  /* .bottom-border {
    border-bottom: 1px solid black;
  } */
</style>
