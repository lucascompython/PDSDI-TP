<script lang="ts">
  import { fileName, currentIndex } from "./stores";
  const { images }: { images: File[] } = $props();
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
            onclick={() => {
              const i = index === 0 ? images.length - 1 : index - 1;
              currentIndex.set(i);
              fileName.set(images[i].name);
            }}
            class="btn btn-circle">❮</a
          >
          <a
            href="#slide{index === images.length - 1 ? 1 : index + 2}"
            onclick={() => {
              const i = index === images.length - 1 ? 0 : index + 1;
              currentIndex.set(i);
              fileName.set(images[i].name);
            }}
            class="btn btn-circle">❯</a
          >
        </div>
      {/if}
    </div>
  {/each}
</div>

<style>
  .carousel-item {
    display: flex;
    flex-direction: column;
  }
</style>
