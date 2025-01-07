<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";

  let { windowLocation }: { windowLocation: URL } = $props();

  const t = useTranslations(getLangFromUrl(windowLocation));

  let dragOver = $state(false);

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    dragOver = false;
    const files = event.dataTransfer?.files;
    // Handle the files
  }

  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    dragOver = true;
  }

  function handleDragLeave() {
    dragOver = false;
  }

  function openFileExplorer() {
    (document.getElementById("fileInput") as HTMLInputElement).click();
  }

  function handleFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const files = input.files;
    // Handle the files
  }
</script>

<div
  class="drag-area {dragOver ? 'drag-over' : ''}"
  ondrop={handleDrop}
  ondragover={handleDragOver}
  ondragleave={handleDragLeave}
  aria-roledescription="Upload files"
  role="button"
  tabindex="0"
>
  <p>{t("upload.drag_drop")}</p>
  <button type="button" onclick={openFileExplorer}>{t("upload.browse")}</button>
  <input id="fileInput" type="file" onchange={handleFileChange} hidden />
</div>

<style>
  .drag-area {
    display: flex;
    flex-direction: column;
    justify-content: center;

    border: 2px dashed var(--text-color);
    border-radius: 4px;

    animation: border-dance 4s infinite linear;

    padding: 20px;
    text-align: center;
    cursor: pointer;
    width: 80%;
    height: 300px;
    margin: 0 auto;
    color: var(--text-color);
  }

  .drag-over {
    border-color: var(--text-color);

    border: none;
    border-radius: none;

    background-image: linear-gradient(
        90deg,
        var(--text-color) 50%,
        transparent 50%
      ),
      linear-gradient(90deg, var(--text-color) 50%, transparent 50%),
      linear-gradient(0deg, var(--text-color) 50%, transparent 50%),
      linear-gradient(0deg, var(--text-color) 50%, transparent 50%);
    background-repeat: repeat-x, repeat-x, repeat-y, repeat-y;
    background-size:
      15px 2px,
      15px 2px,
      2px 15px,
      2px 15px;
    background-position:
      left top,
      right bottom,
      left bottom,
      right top;
    animation: border-dance 0.1s infinite linear;
  }
  @keyframes border-dance {
    0% {
      background-position:
        left top,
        right bottom,
        left bottom,
        right top;
    }

    100% {
      background-position:
        left 15px top,
        right 15px bottom,
        left bottom 15px,
        right top 15px;
    }
  }

  button {
    text-decoration: underline;
  }

  input {
    display: none;
  }

  @media only screen and (max-width: 600px) {
    .drag-area {
      max-width: 20rem;
      max-height: 10rem;
    }
  }
</style>
