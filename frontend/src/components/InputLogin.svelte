<script lang="ts">
  import UsernameLoginIcon from "./Icons/UsernameLoginIcon.svelte";
  import PasswordLoginIcon from "./Icons/PasswordLoginIcon.svelte";
  import {
    getLangFromUrl,
    useTranslatedPath,
    useTranslations,
  } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatePath = useTranslatedPath(lang);

  let isAdmin = "false";
  try {
    isAdmin = localStorage.getItem("isAdmin")!;
  } catch (e) {
    isAdmin = "false";
  }
</script>

<div class="container mx-auto p-4 max-w-lg">
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <UsernameLoginIcon />
    <input type="text" class="grow p-2" placeholder={t("login.username")} />
  </label>
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <PasswordLoginIcon />
    <input
      type="password"
      class="grow p-2"
      placeholder={t("login.password")}
      id="passwordInput"
    />
  </label>

  <a class="link link-hover mb-4">{t("login.forgot.password")}</a>

  <button
    class="btn btn-primary w-full"
    onclick={() => {
      localStorage.setItem("isAdmin", "true");
      console.log(isAdmin);
      window.location.href = translatePath("/");
    }}>{t("login.login")}</button
  >
</div>

<style>
  div {
    display: flex;
    flex-direction: column;
  }
  label {
    display: flex;
    background-color: var(--bg-color);
    margin-top: 1rem;
  }
  a {
    display: flex;
    color: var(--text-color);
    position: relative;
    margin-top: 1rem;
  }
  button {
    background-color: var(--bg-color);
    color: var(--text-color);
    margin-top: 1rem;
  }
  button a {
    margin: auto;
  }
</style>
