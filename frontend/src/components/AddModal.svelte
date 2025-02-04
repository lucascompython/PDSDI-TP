<script lang="ts">
  import { useTranslations, getLangFromUrl } from "src/i18n/utils";
  import EmailIcon from "src/components/Icons/EmailIcon.svg?raw";
  import PasswordLoginIcon from "./Icons/PasswordLoginIcon.svelte";
  import { showAlert, AlertType } from "./Alert/Alert";
  import { onMount } from "svelte";
  import { registerUser } from "src/api/utils";

  const { windowLocation }: { windowLocation: URL } = $props();

  const t = useTranslations(getLangFromUrl(windowLocation));

  let addUser: HTMLDialogElement;
  let nameInput: HTMLInputElement;
  let emailInput: HTMLInputElement;
  let passwordInput: HTMLInputElement;
  let adminCheckbox: HTMLInputElement;

  onMount(() => {
    addUser = document.getElementById("addUser") as HTMLDialogElement;
    nameInput = document.getElementById("nameInput") as HTMLInputElement;
    emailInput = document.getElementById("emailInput") as HTMLInputElement;
    passwordInput = document.getElementById(
      "passwordInput",
    ) as HTMLInputElement;
    adminCheckbox = document.getElementById(
      "admin-checkbox",
    ) as HTMLInputElement;
  });

  function handleAdd(e: MouseEvent) {
    e.preventDefault();

    const name = nameInput.value;
    const email = emailInput.value;
    const password = passwordInput.value;
    const admin = adminCheckbox.checked;

    if (!name || !email || !password) {
      showAlert(t("admin.add.missing"), AlertType.ERROR, addUser);
      return;
    }

    registerUser(name, email, password, admin).then((ok) => {
      if (ok) {
        showAlert(t("admin.add.success"), AlertType.SUCCESS);
      } else {
        showAlert(t("admin.add.error"), AlertType.ERROR);
      }

      nameInput.value = "";
      emailInput.value = "";
      passwordInput.value = "";

      addUser.close();
    });
  }
</script>

<button
  onclick={() => addUser.showModal()}
  class="btn btn-success responsive-btn">{t("admin.add")}</button
>

<dialog id="addUser" class="modal">
  <div class="modal-box bg-color">
    <h3 class="text-lg font-bold">{t("admin.add.add_user")}</h3>
    <label
      class="mt-2 input input-bordered flex items-center gap-2 mb-4 w-full"
    >
      <input type="name" class="grow p-2" placeholder="Name" id="nameInput" />
    </label>
    <label class="input input-bordered flex items-center gap-2 mb-4 w-full">
      {@html EmailIcon}
      <input
        type="email"
        class="grow p-2"
        placeholder="Email"
        id="emailInput"
      />
    </label>
    <label class="input input-bordered flex items-center gap-2 mb-4 w-full">
      <PasswordLoginIcon />
      <input
        type="password"
        class="grow p-2"
        placeholder="Password"
        id="passwordInput"
      />
    </label>
    <label>
      <span class="mr-2">Admin:</span>
      <input
        id="admin-checkbox"
        type="checkbox"
        checked={true}
        class="toggle toggle-info"
      />
    </label>
    <div class="flex justify-between mt-4">
      <form method="dialog">
        <button class="not btn btn-error top">{t("admin.edit.cancel")}</button>
      </form>
      <button onclick={handleAdd} class="not btn btn-success top"
        >{t("admin.edit.update")}</button
      >
    </div>
  </div>
</dialog>

<style>
  button {
    margin-right: 10%;
    max-width: 10%;
  }
  .not {
    max-width: unset;
    margin-right: unset;
  }
</style>
