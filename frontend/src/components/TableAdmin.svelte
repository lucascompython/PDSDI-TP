<script lang="ts">
  import { getLangFromUrl, useTranslations } from "../i18n/utils";
  import { onMount } from "svelte";
  import EmailIcon from "./Icons/EmailIcon.svg?raw";
  import PasswordLoginIcon from "./Icons/PasswordLoginIcon.svelte";

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
</script>

{#if window.innerWidth < 600}
  <div class="overflow-x-auto">
    <table class="table">
      <!-- head -->
      <thead>
        <tr>
          <th>Name</th>
          <th>E-mail</th>
          <th>Admin</th>
          <th> </th>
        </tr>
      </thead>
      <tbody>
        <!-- row 1 -->
        <tr>
          <td>Guilherme</td>
          <td>admin@gmail.com</td>
          <td>Yes</td>
          <th>
            <button
              class="btn btn-xs btn-info link link-hover"
              onclick={() => editUser.showModal()}>Edit</button
            >
            <button
              class="btn btn-xs btn-error link link-hover"
              onclick={() => confirmDelete.showModal()}>Delete</button
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
          <th>Name</th>
          <th>E-mail</th>
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
          <td>Yes</td>
          <th class="flex flex-col space-y-1">
            <button
              class="btn btn-xs btn-info link link-hover"
              onclick={() => editUser.showModal()}>Edit</button
            >
            <button
              class="btn btn-xs btn-error link link-hover"
              onclick={() => confirmDelete.showModal()}>Delete</button
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
    <h3 class="text-lg font-bold">{t("table.admin.delete.title")}</h3>
    <form method="dialog">
      <button class="btn btn-success">Proceed</button>
      <button class="btn btn-error">Cancel</button>
    </form>
  </div>
</dialog>

<dialog id="editUser" class="modal">
  <div class="modal-box bg-color">
    <label class="input input-bordered flex items-center gap-2 mb-4 w-full">
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
    <select class="select select-bordered w-full max-w-xs">
      <option disabled selected>Is Admin?</option>
      <option>Yes</option>
      <option>No</option>
    </select>
    <form method="dialog">
      <button class="btn btn-success top">Confirm</button>
    </form>
  </div>
</dialog>

<style>
  .top {
    margin-top: 1rem;
  }
</style>
