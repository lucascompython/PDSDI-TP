<script lang="ts">
  import { getLangFromUrl, useTranslations } from "../i18n/utils";
  import { onMount } from "svelte";
  import EmailIcon from "./Icons/EmailIcon.svg?raw";
  import PasswordLoginIcon from "./Icons/PasswordLoginIcon.svelte";
  import { showAlert, AlertType } from "./Alert/Alert";

  let { windowLocation }: { windowLocation: URL } = $props();
  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
  let confirmDelete: HTMLDialogElement;
  let editUser: HTMLDialogElement;

  onMount(() => {
    confirmDelete = document.getElementById(
      "confirmDelete",
    ) as HTMLDialogElement;
    editUser = document.getElementById("editUser") as HTMLDialogElement;
  });

  function handleUpdate(event: MouseEvent) {
    event.preventDefault();
    showAlert(t("admin.edit.success"), AlertType.SUCCESS);
    editUser.close();
  }

  function handleDelete(event: MouseEvent) {
    event.preventDefault();
    showAlert(t("admin.delete.success"), AlertType.SUCCESS);
    confirmDelete.close();
  }
</script>

{#if window.innerWidth < 600}
  <div class="overflow-x-auto">
    <table class="table">
      <!-- head -->
      <thead>
        <tr>
          <th>{t("table.name")}</th>
          <th>E-Mail</th>
          <th>Admin</th>
          <th> </th>
        </tr>
      </thead>
      <tbody>
        <!-- row 1 -->
        <tr>
          <td>Guilherme</td>
          <td>admin@gmail.com</td>
          <td>{t("table.yes")}</td>
          <th>
            <button
              class="btn btn-xs btn-info link link-hover"
              onclick={() => editUser.showModal()}>{t("admin.edit")}</button
            >
            <button
              class="btn btn-xs btn-error link link-hover"
              onclick={() => confirmDelete.showModal()}
              >{t("admin.delete")}</button
            >
          </th>
          <td></td>
        </tr>
      </tbody>
    </table>
  </div>
  <style>
    div {
      width: 100%;
    }
    th {
      color: var(--text-color);
      text-align: center;
    }
    td {
      color: var(--text-color);
      font-size: 0.7rem;
    }
    button {
      align-self: center;
    }
  </style>
{:else}
  <div class="overflow-x-auto div-center" style="">
    <table class="table">
      <!-- head -->
      <thead>
        <tr>
          <th> </th>
          <th>{t("table.name")}</th>
          <th>E-Mail</th>
          <th>Admin</th>
          <th></th>
        </tr>
      </thead>
      <tbody>
        <!-- row 1 -->
        <tr>
          <td>1</td>
          <td>Guilherme</td>
          <td>admin@gmail.com</td>
          <td>{t("table.yes")}</td>
          <th class="flex flex-col space-y-1">
            <button
              class="btn btn-xs btn-info link link-hover"
              onclick={() => editUser.showModal()}>{t("admin.edit")}</button
            >
            <button
              class="btn btn-xs btn-error link link-hover"
              onclick={() => confirmDelete.showModal()}
              >{t("admin.delete")}</button
            >
          </th>
        </tr>
      </tbody>
    </table>
  </div>

  <style>
    .div-center {
      width: 1500px;
      margin: 0 auto;
    }
    th {
      color: var(--text-color);
    }
    td {
      color: var(--text-color);
    }
  </style>
{/if}
<dialog id="confirmDelete" class="modal">
  <div class="modal-box bg-color">
    <h3 class="text-lg font-bold">{t("admin.delete.title")}</h3>
    <p class="mb-4">{t("admin.delete.message")}</p>
    <div class="flex justify-between">
      <form method="dialog">
        <button class="btn btn-error">{t("admin.edit.cancel")}</button>
      </form>
      <button onclick={handleDelete} class="btn btn-success"
        >{t("admin.delete")}</button
      >
    </div>
  </div>
</dialog>

<dialog id="editUser" class="modal">
  <div class="modal-box bg-color">
    <h3 class="text-lg font-bold">{t("admin.edit.title")}</h3>
    <label
      class="mt-2 input input-bordered flex items-center gap-2 mb-4 w-full"
    >
      <input
        type="password"
        class="grow p-2"
        placeholder="Name"
        id="passwordInput"
      />
    </label>
    <label class="input input-bordered flex items-center gap-2 mb-4 w-full">
      {@html EmailIcon}
      <input
        type="password"
        class="grow p-2"
        placeholder="Email"
        id="passwordInput"
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
      <input type="checkbox" checked={true} class="toggle toggle-info" />
    </label>
    <div class="flex justify-between">
      <form method="dialog">
        <button class="btn btn-error top">{t("admin.edit.cancel")}</button>
      </form>
      <button onclick={handleUpdate} class="btn btn-success top"
        >{t("admin.edit.update")}</button
      >
    </div>
  </div>
</dialog>

<style>
  .top {
    margin-top: 1rem;
  }
</style>
