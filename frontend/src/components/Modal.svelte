<script lang="ts">
  import AddButtonIcon from "./Icons/AddButtonIcon.svelte";
</script>

<button
  onclick={() =>
    (document.getElementById("my_modal_1") as HTMLDialogElement)?.showModal()}
  ><AddButtonIcon /></button
>
<dialog id="my_modal_1" class="modal scrollbar">
  <div class="modal-box">
    <h3 class="text-lg font-bold">Hello!</h3>
    <p class="py-4">Press ESC key or click outside to close</p>
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
