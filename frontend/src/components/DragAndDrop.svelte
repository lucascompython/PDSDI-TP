<script lang="ts">
  // TODO: see why cant compile to prod with windowLocation SEE ON MOUNT
  // TODO: Optimise the svgs
  // TODO: optimize the t function using stores
  // TODO: See context api

  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  import { fileName, currentIndex } from "./stores";
  import Hanger from "./Icons/Hanger.svg?raw";
  import {
    ClotheCategory,
    Color,
    uploadClothes,
    type Clothe,
  } from "src/api/utils";
  import Carousel from "./Carousel.svelte";
  import { get } from "svelte/store";
  import { onMount } from "svelte";
  import { showAlert, AlertType } from "./Alert/Alert";

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

    currentIndex.subscribe((index) => {
      if (Clothes[index] === undefined) {
        clotheName.value = "";
        clotheCategory.value = t("upload.category");
        clotheColor.value = t("upload.color");
        clotheIsForHotWeather.checked = false;
        return;
      }

      fillData(index);
    });
  });

  function checkClothes(): number {
    if (Clothes.length === 0) {
      return 0;
    }
    for (let i = 0; i < Clothes.length; i++) {
      if (
        !Clothes[i].name ||
        Clothes[i].category === (t("upload.category") as ClotheCategory) ||
        Clothes[i].color === (t("upload.color") as Color)
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
        showAlert(
          `"${file.name}" ${t("upload.file_not_image")}`,
          AlertType.ERROR,
          files.length > 1 ? previewModal : undefined,
        );
        if (files.length === 1) {
          return;
        }
        continue;
      }

      if (file.size > maxFileSize) {
        showAlert(
          `"${file.name}" ${t("upload.file_too_large")}`,
          AlertType.ERROR,
          files.length > 1 ? previewModal : undefined,
        );
        if (files.length === 1) {
          return;
        }
        continue;
      }

      selectedImages.push(file);
      Clothes.push({
        name: "",
        category: t("upload.category") as ClotheCategory,
        color: t("upload.color") as Color,
        isForHotWeather: false,
        image: file,
      });

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

    setData(get(currentIndex));

    const index = checkClothes();
    if (index === -1) {
      uploadClothes(Clothes).then((ok) => {
        if (ok) {
          previewModal.close();
          showAlert(t("upload.success"), AlertType.SUCCESS);
        } else {
          showAlert(t("upload.error"), AlertType.ERROR, previewModal);
        }
      });
    } else {
      showAlert(t("upload.fill_all_fields"), AlertType.WARNING, previewModal);
      currentIndex.set(index);
      window.location.href = `#slide${index + 1}`;
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
        <label class="input input-bordered flex items-center gap-2 w-full">
          {@html Hanger}
          <input
            id="clotheName"
            type="text"
            class="grow w-100"
            placeholder={t("upload.name")}
          />
        </label>
        <div class="flex">
          <select
            id="clotheCategory"
            class="mt-2 mr-2 select select-bordered w-full max-w-xs"
          >
            <option>{t("upload.category")}</option>
            {#each Object.keys(ClotheCategory) as category}
              <option value={category}
                >{t(category.toLowerCase() as any)}</option
              >
            {/each}
          </select>

          <select
            id="clotheColor"
            class="mt-2 select select-bordered w-full max-w-xs"
          >
            <option>{t("upload.color")}</option>
            {#each Object.values(Color) as color}
              <option value={color}>{t(color.toLowerCase() as any)}</option>
            {/each}
          </select>
        </div>
        <label class="label cursor-pointer mb-3 mt-3 flex justify-between">
          <span>{t("upload.is_for_hot_weather")}</span>
          <input id="clotheIsForHotWeather" type="checkbox" class="checkbox" />
        </label>
        <div class="button-container">
          <form
            method="dialog"
            onsubmit={() => {
              selectedImages.length = 0;
              Clothes.length = 0;
            }}
          >
            <!-- if there is a button in form, it will close the modal -->
            <button class="btn">{t("upload.cancel")}</button>
          </form>
          <div class="right-buttons">
            <button
              class="btn btn-error w-full"
              onclick={() => {
                let index = get(currentIndex);
                Clothes.splice(index, 1);
                selectedImages.splice(index, 1);

                if (selectedImages.length === 0) {
                  previewModal.close();
                } else {
                  currentIndex.update((n) => {
                    // if we're on the last image then go back one otherwise go the right (current index since we removed the current image)
                    if (n === selectedImages.length) {
                      index = n - 1;
                      return index;
                    }
                    index = n;
                    return n;
                  });

                  fileName.set(selectedImages[index].name);
                }
              }}>{t("upload.remove")}</button
            >
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
  .modal {
    /* Change this display because daisyui defines it as grid and it breaks with the alert popup */
    display: flex;
    justify-content: center;
    align-items: center;
  }
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
