<script lang="ts">
  import { get } from "svelte/store";
  import SettingsIcon from "./Icons/SettingsIcon.svelte";
  import { isAdmin } from "./store";

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

<div class="dropdown dropdown-end">
  <div
    tabindex="0"
    role="button"
    class="m-1"
    onmousedown={(event) => {
      event.preventDefault();
      const element = event.currentTarget;
      if (document.activeElement === element) {
        element.blur();
      } else {
        element.focus();
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
    {#if get(isAdmin)}
      <li>
        <button
          class="btn"
          onclick={() => {
            window.location.href = translatePath("/admin");
          }}>{t("settings.admin")}</button
        >
      </li>
    {/if}
    <li>
      <button
        class="btn btn-error"
        onclick={() => {
          window.location.href = translatePath("/login");
          isAdmin.set(false);
        }}>{t("settings.logout")}</button
      >
    </li>
  </ul>
</div>

<style>
  ul {
    background-color: none;
  }
</style>
