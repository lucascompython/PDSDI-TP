<script lang="ts">
  import EmailIcon from "./Icons/EmailIcon.svelte";
  import PasswordLoginIcon from "./Icons/PasswordLoginIcon.svelte";
  import ErrorAlert from "./ErrorAlert.svelte"; // Cannot remove this for some reason
  import GenericAvatarIcon from "./Icons/GenericAvatarIcon.svelte";
  import { isErrorVisible } from "./stores";
  import {
    getLangFromUrl,
    useTranslatedPath,
    useTranslations,
  } from "src/i18n/utils";

  import { loginUser } from "src/api/utils";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatePath = useTranslatedPath(lang);

  let email = $state("");
  let password = $state("");

  function showErrorAlert() {
    if (!$isErrorVisible) {
      $isErrorVisible = true;
      const errorAlert = document.createElement("error-alert");
      errorAlert.setAttribute("message", t("login.error"));
      document.body.appendChild(errorAlert);
      setTimeout(() => {
        $isErrorVisible = false;
        errorAlert.remove();
      }, 5000);
    }
  }

  async function handleLogin(event: SubmitEvent) {
    event.preventDefault();
    if (email === "" || password === "") {
      showErrorAlert();
      return;
    }

    const resp = await loginUser(email, password);

    if (!resp) {
      showErrorAlert();
    } else {
      window.location.href = translatePath("/");
    }
  }
</script>

<GenericAvatarIcon />

<!-- TODO: Add input validation errors -->

<form class="container mx-auto p-4 max-w-lg" onsubmit={handleLogin}>
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <EmailIcon />
    <input
      bind:value={email}
      type="email"
      class="grow p-2"
      placeholder={t("login.email")}
    />
  </label>
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <PasswordLoginIcon />
    <input
      bind:value={password}
      type="password"
      class="grow p-2"
      placeholder={t("login.password")}
      id="passwordInput"
    />
  </label>

  <a class="link link-hover mb-4">{t("login.forgot.password")}</a>

  <button class="btn btn-primary w-full">{t("login.login")}</button>
</form>

<style>
  div {
    display: flex;
    flex-direction: column;

    align-items: center;
    justify-content: center;
  }

  input {
    background-color: var(--bg-color);
    color: var(--text-color);
  }

  label {
    display: flex;
    background-color: var(--bg-color);
    margin-top: 1rem;
    width: 100%;
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
