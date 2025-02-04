<script lang="ts">
  import type { useTranslations } from "src/i18n/utils";
  import {
    numberToCategory,
    numberToColor,
    type ClotheResponse,
  } from "../api/utils";

  const {
    clothe,
    t,
  }: { clothe: ClotheResponse; t: ReturnType<typeof useTranslations> } =
    $props();
</script>

<div class="card bg-base-100 mwidth shadow-xl">
  <figure>
    <img
      src={URL.createObjectURL(new Blob([clothe.file]))}
      alt={clothe.file_name}
    />
  </figure>
  <div class="card-body">
    <h2 class="card-title">{clothe.name}</h2>
    <div class="card-actions">
      <div class="badge-container center-badge">
        <div class="badge badge-outline">
          {numberToCategory(clothe.category)}
        </div>
        <div class="badge badge-outline">{numberToColor(clothe.color)}</div>
        <div class="badge badge-outline">
          {t("upload.is_for_hot_weather")}: {clothe.is_for_hot_weather}
        </div>
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

  .mwidth {
    width: 100%;
  }

  @media only screen and (max-width: 1210px) {
    .mwidth {
      width: calc(var(--spacing) * 40);
    }

    .badge {
      font-size: 0.7rem;
      margin-bottom: 0.5rem;
    }
    .badge-outline:nth-child(3) {
      height: 3.5rem;
    }
  }
</style>
