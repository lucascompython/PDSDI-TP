<script lang="ts">
  import { type ClotheResponse } from "../api/utils";

  const {
    clothe,
    image,
  }: { clothe: ClotheResponse; image: { filename: string; data: Blob } } =
    $props();

  let imageUrl: string;
  if (image && image.filename) {
    imageUrl = URL.createObjectURL(image.data);
    console.log("imageUrl", imageUrl);
  }
</script>

<div class="card bg-base-100 w-96 shadow-xl">
  <figure>
    <img src={imageUrl} alt={image.filename} />
  </figure>
  <div class="card-body">
    <h2 class="card-title">Shoes!</h2>
    <div class="card-actions">
      <div class="badge-container center-badge">
        <div class="badge badge-outline">{clothe.category}</div>
        <div class="badge badge-outline">{clothe.color}</div>
        <div class="badge badge-outline">{clothe.is_hot_weather}</div>
      </div>
    </div>
  </div>
</div>

<style>
  .center-badge {
    width: 100%;
  }
  div {
    background-color: var(--text-color);
    color: var(--bg-color);
    border-radius: 1rem;
  }
</style>
