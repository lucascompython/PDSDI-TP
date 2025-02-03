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
  import { onMount } from "svelte";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  const translatePath = useTranslatedPath(lang);

  let email = $state("");
  let password = $state("");
  let cookieModal: HTMLDialogElement;
  let forgotPassword: HTMLDialogElement;

  onMount(() => {
    cookieModal = document.getElementById("cookie_modal") as HTMLDialogElement;
    forgotPassword = document.getElementById(
      "forgotPassword",
    ) as HTMLDialogElement;
  });

  async function handleLogin(event: SubmitEvent) {
    event.preventDefault();

    if (cookieModal.open || forgotPassword.open) {
      return;
    }

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
  <button class="btn btn-primary w-full">{t("login.login")}</button>
</form>

<button
  class="link link-hover disable_background"
  onclick={() => forgotPassword.showModal()}
>
  {t("login.forgot.password")}</button
>

<button
  class="link link-hover mb-4 disable_background"
  onclick={() => cookieModal.showModal()}
>
  {t("login.cookies.label")}</button
>

<dialog id="forgotPassword" class="modal">
  <div class="modal-box bg-color">
    <label class="input input-bordered flex items-center gap-2 mb-4">
      {@html EmailIcon}
      <input type="email" class="grow p-2" placeholder={t("login.email")} />
    </label>
    <form method="dialog">
      <button class="btn">{t("login.forgot.submit")}</button>
    </form>
  </div>
</dialog>

<dialog id="cookie_modal" class="modal">
  <div class="modal-box bg-color">
    <h3 class="text-lg font-bold">{t("login.cookies.title")}</h3>
    <p class="py-4">{t("login.cookies")}</p>
    <div class="modal-action">
      <form method="dialog">
        <button class="btn">Ok</button>
      </form>
    </div>
  </div>
</dialog>

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
    border-color: var(--text-color);
  }
  p {
    color: var(--text-color);
  }

  .disable_background {
    background-color: transparent;
  }
  button {
    background-color: var(--bg-color);
    color: var(--text-color);
    margin-top: 1rem;
  }
</style>
