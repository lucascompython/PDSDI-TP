<script lang="ts">
  import SettingsIcon from "./Icons/SettingsIcon.svelte";
  import { writable } from "svelte/store";
  import {
    getLangFromUrl,
    useTranslatedPath,
    useTranslations,
  } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatePath = useTranslatedPath(lang);

  let isAdmin = writable(false);
</script>

<!-- Faltam Merdas -->

<div class="dropdown dropdown-end">
  <div
    tabindex="0"
    role="button"
    class="m-1"
    onmousedown={(event) => {
      const element = event.currentTarget;
      if (document.activeElement === element) {
        element.removeAttribute("tabindex");
      } else {
        element.tabIndex = 0;
      }
    }}
  >
    <SettingsIcon />
  </div>
  <ul
    tabindex="-1"
    id="dropdown-settings"
    class="menu dropdown-content bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
  >
    <li>
      <button
        class="btn"
        onclick={() => (window.location.href = translatePath("/profile"))}
        >{t("settings.profile")}</button
      >
    </li>
    {#if isAdmin}
      <li>
        <button
          class="btn"
          onclick={() => (window.location.href = translatePath("/admin"))}
          >{t("settings.admin")}</button
        >
      </li>
    {/if}
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
    background-color: none;
  }
</style>
