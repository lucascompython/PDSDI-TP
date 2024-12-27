<script lang="ts">
  import SettingsIcon from "./SettingsIcon.svelte";
  import { getLangFromUrl, useTranslatedPath } from "src/i18n/utils";
  const translatePath = useTranslatedPath(
    getLangFromUrl(window.location.pathname),
  );
</script>

<!-- Faltam Merdas -->

<button
  onclick={() =>
    (document.getElementById("my_modal_2") as HTMLDialogElement)?.showModal()}
  ><SettingsIcon /></button
>
<dialog id="my_modal_2" class="modal">
  <div class="modal-box">
    <h3 class="text-lg font-bold">Hello!</h3>
    <p class="py-4">Press ESC key or click outside to close</p>
    <button
      class="btn btn-error"
      onclick={() => (window.location.href = translatePath("/login"))}
      >Logout</button
    >
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>

<style>
  button {
    background-color: none;
    border: none;
    border-radius: none;
  }

  button:hover {
    background-color: none;
  }
</style>
