<script lang="ts">
  import { fileName, clothes, t as tStore } from "./stores";
  import Hanger from "./Icons/Hanger.svelte";
  import { ClotheCategory } from "src/api/utils";
  const { images }: { images: File[] } = $props();

  const t = $tStore;
</script>

<div class="carousel w-full">
  {#each images as image, index}
    <div id="slide{index + 1}" class="carousel-item relative w-full">
      <img
        src={URL.createObjectURL(image)}
        class="w-full object-cover"
        alt="image{index}"
      />
      {#if images.length > 1}
        <div
          class="absolute left-5 right-5 top-1/2 flex -translate-y-1/2 transform justify-between"
        >
          <a
            href="#slide{index === 0 ? images.length : index}"
            onclick={() =>
              fileName.set(
                images[index === 0 ? images.length - 1 : index - 1].name,
              )}
            class="btn btn-circle">❮</a
          >
          <a
            href="#slide{index === images.length - 1 ? 1 : index + 2}"
            onclick={() =>
              fileName.set(
                images[index === images.length - 1 ? 0 : index + 1].name,
              )}
            class="btn btn-circle">❯</a
          >
        </div>
      {/if}
      <div class="form-container">
        <label class="input input-bordered flex items-center gap-2">
          <Hanger />
          <input type="text" class="grow" placeholder={t("upload.name")} />
        </label>
        <select class="select select-bordered w-full max-w-xs">
          <option>{t("upload.category")}</option>
          {#each Object.keys(ClotheCategory) as category}
            <option>{category}</option>
          {/each}
        </select>
      </div>
    </div>
  {/each}
</div>

<style>
  .carousel-item {
    display: flex;
    flex-direction: column;
  }
  .form-container {
    margin-top: 1rem;
    flex-direction: column;
  }
</style>
