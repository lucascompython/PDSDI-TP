<script lang="ts">
  import { onMount } from "svelte";
  import { type ClotheResponse, getClothes } from "../api/utils";
  import EyeCard from "./EyeCard.svelte";

  let clothes: ClotheResponse[] = $state([]);
  let images: { [key: string]: Blob } = $state({});

  onMount(() => {
    console.log("CardLoader mounted");

    getClothes().then((response) => {
      clothes = response.clothes;
      images = response.images;
      // console.log("clothes", clothes);
      console.log("images", images);

      console.log("getImagesForClothe", getImageForClothe(clothes[0]));
    });
  });

  function getImageForClothe(clothe: ClotheResponse): {
    filename: string;
    data: Blob;
  } {
    const filename = `${clothe.user_id}-${clothe.name}.png`;
    const data = images[filename];
    return { filename, data };
  }
</script>

{#each clothes as clothe, i}
  <EyeCard {clothe} image={getImageForClothe(clothe)!} />
{/each}
