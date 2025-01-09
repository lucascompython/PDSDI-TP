<script lang="ts">
  import { getLangFromUrl, useTranslations } from "src/i18n/utils";
  let { windowLocation }: { windowLocation: URL } = $props();

  const lang = getLangFromUrl(windowLocation);
  const t = useTranslations(lang);
</script>

<span class="text-lg">{t("colorO.label")}</span>

{#if window.innerWidth < 600}
  <div>
    <div>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FF0000] [--chkfg:black]"
        />
        {t("colorO.red")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFFFFF] [--chkfg:black]"
        />
        {t("colorO.white")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#000000] [--chkfg:white]"
        />
        {t("colorO.black")}
      </label>
    </div>
    <div>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFC0CB] [--chkfg:black]"
        />
        {t("colorO.pink")}
      </label>

      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#800080] [--chkfg:black]"
        />
        {t("colorO.purple")}
      </label>

      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFFF00] [--chkfg:black]"
        />
        {t("colorO.yellow")}
      </label>
    </div>
    <div>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFA500] [--chkfg:black]"
        />
        {t("colorO.orange")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#808080] [--chkfg:black]"
        />
        {t("colorO.grey")}
      </label>

      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#008000] [--chkfg:black]"
        />
        {t("colorO.green")}
      </label>
    </div>
    <div>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#0000FF] [--chkfg:black]"
        />
        {t("colorO.blue")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#A52A2A] [--chkfg:black]"
        />
        {t("colorO.brown")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFD700] [--chkfg:black]"
        />
        {t("colorO.gold")}
      </label>
    </div>
  </div>
{:else}
  <div>
    <div>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FF0000] [--chkfg:black]"
        />
        {t("colorO.red")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFFFFF] [--chkfg:black]"
        />
        {t("colorO.white")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#000000] [--chkfg:white]"
        />
        {t("colorO.black")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFC0CB] [--chkfg:black]"
        />
        {t("colorO.pink")}
      </label>
    </div>
    <div>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#800080] [--chkfg:black]"
        />
        {t("colorO.purple")}
      </label>

      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFFF00] [--chkfg:black]"
        />
        {t("colorO.yellow")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFA500] [--chkfg:black]"
        />
        {t("colorO.orange")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#808080] [--chkfg:black]"
        />
        {t("colorO.grey")}
      </label>
    </div>
    <div>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#008000] [--chkfg:black]"
        />
        {t("colorO.green")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#0000FF] [--chkfg:black]"
        />
        {t("colorO.blue")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#A52A2A] [--chkfg:black]"
        />
        {t("colorO.brown")}
      </label>
      <label class="fieldset-label">
        <input
          type="checkbox"
          checked={false}
          class="checkbox [--chkbg:#FFD700] [--chkfg:black]"
        />
        {t("colorO.gold")}
      </label>
    </div>
  </div>
{/if}
<div class="center-button">
  <button class="btn btn-xs md:btn-md btn-wide lg:btn-lg"
    >{t("outfit.button")}</button
  >
</div>

<style>
  span {
    color: var(--text-color);
  }
  div {
    display: flex;
    flex-direction: column;
  }
  div div {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    max-width: 97%;
    margin-top: 2rem;
  }
  label {
    align-items: center;
    display: flex;
    min-width: 110px;
    color: var(--text-color);
  }
  button {
    margin-top: 5rem;
    max-width: 200px;
    background-color: var(--text-color);
    color: var(--bg-color);
  }
  button:hover {
    background-color: var(--text-color);
    color: var(--bg-color);
  }
  input {
    border-color: var(--text-color);
    margin-right: 0.5rem;
  }
  .center-button {
    display: flex;
    width: 100%;
    justify-content: center;
    align-items: center;
  }
  @media only screen and (max-width: 600px) {
    div div {
      margin-top: 1rem;
    }
    label {
      min-width: 80px;
      font-size: 0.7rem;
    }
    button {
      margin-top: 2rem;
      min-height: 3rem;
    }
  }
</style>
