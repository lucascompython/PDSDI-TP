<script lang="ts">
  import SettingsIcon from "./Icons/SettingsIcon.svelte";
  import { logoutUser } from "src/api/utils";
  import { isAdmin as isAdminStore } from "./stores";
  import { onMount } from "svelte";

  import {
    getLangFromUrl,
    useTranslatedPath,
    useTranslations,
  } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatePath = useTranslatedPath(lang);

  let isAdmin = $state(false);
  isAdminStore.subscribe((value) => {
    // we must do this because the client-side middleware can finish after the component is mounted
    isAdmin = value;
  });
  let isRotated = false;

  let settingsIcon: HTMLElement;

  onMount(() => {
    settingsIcon = document.getElementById("dropdown-settings")!;
  });
</script>

<div class="dropdown dropdown-end">
  <div
    tabindex="0"
    role="button"
    class="m-1"
    onmousedown={(event) => {
      event.preventDefault();

      if (isRotated) {
        settingsIcon.style.transform = "rotate(0deg)";
        isRotated = false;
      } else {
        settingsIcon.style.transform = "rotate(90deg)";
        isRotated = true;
      }

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
    {#if isAdmin}
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
          logoutUser().then(() => {
            window.location.href = translatePath("/login");
          });
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
