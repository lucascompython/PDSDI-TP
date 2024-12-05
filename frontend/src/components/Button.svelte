<script>
  import AddButtonIcon from "./AddButtonIcon.svelte";
  import Modal from "./Modal.svelte";
  let showModal = $state(false); // HTMLDialogElement
</script>

<button onclick={() => (showModal = true)}>
  <AddButtonIcon />
</button>

<!-- Lembrar Traduções Dentro Modal -->

<Modal bind:showModal>
  {#snippet header()}
    <h2 class="color">
      modal
      <small><em>adjective</em> modal</small>
    </h2>
  {/snippet}

  <ol class="definition-list color">
    <li>of or relating to modality in logic</li>
    <li>
      containing provisions as to the mode of procedure or the manner of taking
      effect —used of a contract or legacy
    </li>
    <li>of or relating to a musical mode</li>
    <li>of or relating to structure as opposed to substance</li>
    <li>
      of, relating to, or constituting a grammatical form or category
      characteristically indicating predication
    </li>
    <li>of or relating to a statistical mode</li>
  </ol>

  <a href="https://www.merriam-webster.com/dictionary/modal"
    >merriam-webster.com</a
  >
</Modal>

<style>
  button {
    display: block;
    margin: auto;
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
  }
  .color {
    color: var(--text-color);
  }
  li {
    margin-bottom: 0.5em;
  }
</style>
