<script lang="ts">
  import EmailIcon from "./Icons/EmailIcon.svg?raw";
  import PasswordLoginIcon from "./Icons/PasswordLoginIcon.svelte";
  import GenericAvatarIcon from "./Icons/GenericAvatarIcon.svelte";
  import {
    getLangFromUrl,
    useTranslatedPath,
    useTranslations,
  } from "src/i18n/utils";

  import { loginUser } from "src/api/utils";
  import { showAlert, AlertType } from "./Alert/Alert";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatePath = useTranslatedPath(lang);

  let email = $state("");
  let password = $state("");

  async function handleLogin(event: SubmitEvent) {
    event.preventDefault();
    if (email === "" || password === "") {
      showAlert(t("login.error"), AlertType.ERROR);
      return;
    }

    const resp = await loginUser(email, password);

    if (!resp) {
      showAlert(t("login.error"), AlertType.ERROR);
    } else {
      window.location.href = translatePath("/");
    }
  }
</script>

<GenericAvatarIcon />

<!-- TODO: Add input validation errors -->
<!-- TODO: Optimize this by removing bind:value -->

<form class="container mx-auto p-4 max-w-lg" onsubmit={handleLogin}>
  <label class="input input-bordered flex items-center gap-2 mb-4">
    {@html EmailIcon}
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

  <div class="relative">
    <a class="link link-hover mb-4" href="#forgot_password_modal"
      >{t("login.forgot.password")}</a
    >
    <a class="link link-hover mb-4 absolute" href="#cookies_modal"
      >{t("login.cookies.label")}</a
    >
  </div>
  <button class="btn btn-primary w-full">{t("login.login")}</button>
</form>

<div class="modal" role="dialog" id="forgot_password_modal">
  <div class="modal-box bg-color">
    <label class="input input-bordered flex items-center gap-2 mb-4">
      {@html EmailIcon}
      <input type="email" class="grow p-2" placeholder={t("login.email")} />
    </label>
    <div class="modal-action">
      <!-- svelte-ignore a11y_invalid_attribute -->
      <a href="" class="btn modalButton">{t("login.forgot.submit")}</a>
    </div>
  </div>
</div>

<div class="modal bg-color" role="dialog" id="cookies_modal">
  <div class="modal-box bg-color">
    <p>{t("login.cookies")}</p>
    <div class="modal-action">
      <!-- svelte-ignore a11y_invalid_attribute -->
      <a href="" class="btn modalButton">Ok</a>
    </div>
  </div>
</div>

<style>
  div {
    display: flex;
    flex-direction: column;

    align-items: center;
    justify-content: center;
  }

  .bg-color {
    background-color: var(--bg-color);
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
  p {
    color: var(--text-color);
  }
  .modalButton {
    color: var(--text-color);
    background-color: var(--bg-color);
  }
  button {
    background-color: var(--bg-color);
    color: var(--text-color);
    margin-top: 1rem;
  }
</style>
