<script lang="ts">
  // TODO: see why cant compile to prod with windowLocation SEE ON MOUNT
  // TODO: Optimise the svgs
  // TODO: optimize the t function using stores
  // TODO: See context api

  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  import { fileName, currentIndex } from "./stores";
  import Hanger from "./Icons/Hanger.svg?raw";
  import { ClotheCategory, Color, type Clothe } from "src/api/utils";
  import Carousel from "./Carousel.svelte";
  import { get } from "svelte/store";
  import { onMount } from "svelte";

  let { windowLocation }: { windowLocation: URL } = $props();

  const t = useTranslations(getLangFromUrl(windowLocation));

  let dragOver = $state(false);

  const selectedImages: File[] = $state([]);
  const Clothes: Clothe[] = $state([]);

  const maxFileSize = 10 * 1024 * 1024; // 10MB

  let clotheName: HTMLInputElement;
  let clotheCategory: HTMLSelectElement;
  let clotheColor: HTMLSelectElement;
  let clotheIsForHotWeather: HTMLInputElement;
  let previewModal: HTMLDialogElement;
  let fileInput: HTMLInputElement;

  onMount(() => {
    clotheName = document.getElementById("clotheName") as HTMLInputElement;
    clotheCategory = document.getElementById(
      "clotheCategory",
    ) as HTMLSelectElement;
    clotheColor = document.getElementById("clotheColor") as HTMLSelectElement;
    clotheIsForHotWeather = document.getElementById(
      "clotheIsForHotWeather",
    ) as HTMLInputElement;

    previewModal = document.getElementById("previewModal") as HTMLDialogElement;
    fileInput = document.getElementById("fileInput") as HTMLInputElement;
  });

  currentIndex.subscribe((index) => {
    console.log("current index:", index);

    if (!shouldUpdateValues(index)) {
      return;
    }

    fillData(index);
  });

  function shouldUpdateValues(index: number): boolean {
    if (Clothes[index] === undefined) {
      return false;
    }

    return (
      clotheName.value !== Clothes[index].name ||
      clotheCategory.value !== Clothes[index].category ||
      clotheColor.value !== Clothes[index].color ||
      clotheIsForHotWeather.checked !== Clothes[index].isForHotWeather
    );
  }

  /* function that checks if all the Clothes items properties are filled returns the index of the first item that is not filled */
  function checkClothes(): number {
    for (let i = 0; i < Clothes.length; i++) {
      if (
        !Clothes[i].name ||
        !Clothes[i].category ||
        !Clothes[i].color ||
        Clothes[i].isForHotWeather === undefined
      ) {
        return i;
      }
    }
    return -1;
  }

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

      previewModal.showModal();
    }
  }

  function handleFileChange(event: Event) {
    const input = event.target as HTMLInputElement;
    const files = input.files;
    if (files) {
      processFiles(files);
    }
  }

  function fillData(index: number) {
    if (!shouldUpdateValues(index)) {
      return;
    }

    console.log("Clothes[index]:", Clothes[index]);

    clotheName.value = Clothes[index].name;
    clotheCategory.value = Clothes[index].category;
    clotheColor.value = Clothes[index].color;
    clotheIsForHotWeather.checked = Clothes[index].isForHotWeather;
  }

  function setData(index: number) {
    Clothes[index] = {
      name: clotheName.value,
      category: clotheCategory.value as ClotheCategory,
      color: clotheColor.value as Color,
      isForHotWeather: clotheIsForHotWeather.checked,
      image: selectedImages[index],
    };
  }

  function handleUpload(event?: MouseEvent) {
    if (event) {
      event.preventDefault();
    }

    console.log("CurrentIndex in handleUpload:", get(currentIndex));

    const index = checkClothes();
    if (index !== -1) {
      fillData(get(currentIndex));
      // Clothes[index] = {
      //   name: clotheName.value,
      //   category: clotheCategory.value as ClotheCategory,
      //   color: clotheColor.value as Color,
      //   isForHotWeather: clotheIsForHotWeather.checked,
      //   image: selectedImages[0],
      // };
      console.log("Clothes:", Clothes);
    } else {
      Clothes.push({
        name: clotheName.value,
        category: clotheCategory.value as ClotheCategory,
        color: clotheColor.value as Color,
        isForHotWeather: clotheIsForHotWeather.checked,
        image: selectedImages[0],
      });
    }

    // previewModal.close();
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
  <button type="button" onclick={() => fileInput.click()}
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

    <Carousel images={selectedImages} {setData} />

    <div class="modal-action">
      <div class="form-container">
        <label class="m-2 input input-bordered flex items-center gap-2">
          {@html Hanger}
          <input
            id="clotheName"
            type="text"
            class="grow"
            placeholder={t("upload.name")}
          />
        </label>
        <div class="flex">
          <select
            id="clotheCategory"
            class="m-2 select select-bordered w-full max-w-xs"
          >
            <option>{t("upload.category")}</option>
            {#each Object.values(ClotheCategory) as category}
              <option value={category}>{t(category as any)}</option>
            {/each}
          </select>

          <select
            id="clotheColor"
            class="m-2 select select-bordered w-full max-w-xs"
          >
            <option>{t("upload.color")}</option>
            {#each Object.values(Color) as color}
              <option value={color}>{t(color as any)}</option>
            {/each}
          </select>
        </div>
        <label class="label cursor-pointer m-2">
          <span class="label-text">{t("upload.is_for_hot_weather")}</span>
          <input id="clotheIsForHotWeather" type="checkbox" class="checkbox" />
        </label>
        <div class="button-container">
          <form
            method="dialog"
            onsubmit={() => {
              selectedImages.length = 0;
            }}
          >
            <!-- if there is a button in form, it will close the modal -->
            <button class="btn">{t("upload.cancel")}</button>
          </form>
          <div class="right-buttons">
            <button class="btn btn-error w-full">{t("upload.remove")}</button>
            <button class="btn btn-accent w-full" onclick={handleUpload}
              >{t("upload.upload")}</button
            >
          </div>
        </div>
      </div>
    </div>
  </div>
</dialog>

<style>
  .form-container {
    margin-top: 1rem;
    flex-direction: column;
  }
  .modal-action {
    display: block;
  }

  .button-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    width: 100%;
  }

  .right-buttons {
    display: flex;
    gap: 1rem;
  }
  .btn {
    flex: 1;
  }

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

  #fileInput {
    display: none;
  }

  @media only screen and (max-width: 600px) {
    .drag-area {
      max-width: 20rem;
      max-height: 10rem;
    }
  }
</style>
