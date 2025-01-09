<script lang="ts">
  import EmailIcon from "./Icons/EmailIcon.svelte";
  import PasswordLoginIcon from "./Icons/PasswordLoginIcon.svelte";
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
</script>

<div class="container mx-auto p-4 max-w-lg">
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <EmailIcon />
    <input
      type="email"
      class="grow p-2"
      placeholder={t("profile.new_email")}
      value="lucas@gmail.com"
    />
  </label>
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <PasswordLoginIcon />
    <input
      type="password"
      class="grow p-2"
      placeholder={t("profile.new_password")}
      id="passwordInput"
    />
  </label>
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <PasswordLoginIcon />
    <input
      type="password"
      class="grow p-2"
      placeholder={t("profile.repeat_password")}
      id="passwordInput"
    />
  </label>

  <button class="btn btn-primary w-full">{t("profile.confirm_changes")}</button>
</div>

<style>
  div {
    display: flex;
    flex-direction: column;
  }
  label {
    display: flex;
    background-color: var(--bg-color);
  }
  button {
    background-color: var(--bg-color);
    color: var(--text-color);
    margin-top: 1rem;
  }
</style>
