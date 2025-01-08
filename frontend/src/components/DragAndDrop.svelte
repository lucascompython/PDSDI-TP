<script lang="ts">
  // TODO: see why cant compile to prod with windowLocation
  // TODO: Optimise the svgs

  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  import { fileName, clothes, t as tStore } from "./stores";
  import Carousel from "./Carousel.svelte";

  let { windowLocation }: { windowLocation: URL } = $props();

  const t = useTranslations(getLangFromUrl(windowLocation));
  tStore.set(t);

  let dragOver = $state(false);

  let selectedImages: File[] = $state([]);

  const maxFileSize = 10 * 1024 * 1024; // 10MB

  function handleDrop(event: DragEvent) {
    event.preventDefault();
    dragOver = false;
    const files = event.dataTransfer?.files;

    if (files) {
      processFiles(files);
    }
  }

  function processFiles(files: FileList) {
    for (let i = 0; i < files.length; i++) {
      const file = files[i];

      if (!file.type.startsWith("image/")) {
        console.error("File is not an image");
        continue;
      }

      if (file.size > maxFileSize) {
        console.error("File is too large");
        continue;
      }
      selectedImages.push(file);

      fileName.set(selectedImages[0].name);

      (
        document.getElementById("previewModal") as HTMLDialogElement
      )?.showModal();
    }
  }

  function handleFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const files = input.files;
    if (files) {
      processFiles(files);
    }
  }
</script>

<div
  class="drag-area {dragOver ? 'drag-over' : ''}"
  ondrop={handleDrop}
  aria-roledescription="Upload files"
  role="button"
  tabindex="0"
>
  <p>{t("upload.drag_drop")}</p>
  <button
    type="button"
    onclick={() =>
      (document.getElementById("fileInput") as HTMLInputElement).click()}
    >{t("upload.browse")}</button
  >
  <input
    id="fileInput"
    type="file"
    accept="image/*"
    multiple
    onchange={handleFileChange}
    hidden
  />
</div>
<dialog id="previewModal" class="modal">
  <div class="modal-box">
    <h3 class="text-lg font-bold">{t("upload.preview")}</h3>
    <p class="py-4">{$fileName}</p>

    <Carousel images={selectedImages} />

    <div class="modal-action">
      <form
        method="dialog"
        onsubmit={() => {
          selectedImages.length = 0;
        }}
      >
        <!-- if there is a button in form, it will close the modal -->
        <button class="btn">{t("upload.cancel")}</button>
      </form>
    </div>
  </div>
</dialog>

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
