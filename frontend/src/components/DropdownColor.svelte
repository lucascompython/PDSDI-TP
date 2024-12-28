<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  let { windowLocation }: { windowLocation: URL } = $props();

  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
</script>

<details class="dropdown">
  <summary class="btn m-1">{t("colorO.dropdownLabel")}</summary>
  <ul
    class="menu dropdown-content bg-base-100 rounded-box z-[1] w-52 p-2 shadow"
  >
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.red")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#FF0000] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.white")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#FFFFFF] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.blue")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#0000FF] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.black")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:black] [--chkfg:white]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.brown")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#8B4513] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.pink")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#FFC0CB] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.purple")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#800080] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.yellow")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#FFFF00] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.orange")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#FFA500] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
    <li>
      <div class="form-control">
        <label class="label cursor-pointer">
          <span class="label-text mr-1">{t("colorO.grey")}-</span>
          <input
            type="checkbox"
            checked={true}
            class="checkbox [--chkbg:#808080] [--chkfg:black]"
          />
        </label>
      </div>
    </li>
  </ul>
</details>

<style>
  ul {
    background-color: none;
  }
  :global(.dark) summary {
    background-color: #0d1117;
  }
  :global(.dark) summary:hover {
    background-color: #0d1117;
  }
  summary {
    background-color: white;
    border: 1px solid gray;
    color: var(--text-color);
  }
  summary:hover {
    background-color: white;
  }
  span {
    color: var(--text-color);
  }
</style>
