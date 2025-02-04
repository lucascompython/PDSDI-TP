<script lang="ts">
  import { OutfitType } from "src/api/bool_pack";
  import type { Outfit } from "src/api/utils";
  import { useTranslations } from "src/i18n/utils";

  const {
    outfit,
    image,
    t,
  }: {
    outfit: Outfit;
    image: Blob;
    t: ReturnType<typeof useTranslations>;
  } = $props();

  function formatDate(dateString: string): string {
    const date = new Date(dateString);
    const day = String(date.getDate()).padStart(2, "0");
    const month = String(date.getMonth() + 1).padStart(2, "0");
    const year = date.getFullYear();
    return `${day}-${month}-${year}`;
  }
</script>

<div class="card bg-base-100 mwidth shadow-xl">
  <figure>
    <img src={URL.createObjectURL(image)} alt={outfit.name} />
  </figure>
  <div class="card-body">
    <h2 class="card-title">{outfit.name}</h2>
    <div class="card-actions">
      <div class="badge-container center-badge">
        <div class="badge badge-outline">
          {t(OutfitType[outfit.outfit_type] as any)}
        </div>
        <div class="badge badge-outline">{formatDate(outfit.created_at)}</div>
      </div>
    </div>
  </div>
</div>
