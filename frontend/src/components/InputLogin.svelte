<script lang="ts">
  import UsernameLoginIcon from "./UsernameLoginIcon.svelte";
  import PasswordLoginIcon from "./PasswordLoginIcon.svelte";
  import { getLangFromUrl, useTranslatedPath } from "src/i18n/utils";
  const translatePath = useTranslatedPath(
    getLangFromUrl(window.location.pathname),
  );
</script>

<div class="container mx-auto p-4 max-w-lg">
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <UsernameLoginIcon />
    <input type="text" class="grow p-2" placeholder="Username" />
  </label>
  <label class="input input-bordered flex items-center gap-2 mb-4">
    <PasswordLoginIcon />
    <input
      type="password"
      class="grow p-2"
      placeholder="Password"
      id="passwordInput"
    />
  </label>

  <a class="link link-hover mb-4">I'm a simple link</a>

  <!-- HREF TRANSLATE PATH SVELTE -->

  <button
    class="btn btn-primary w-full"
    onclick={() => (window.location.href = translatePath("/"))}>Login</button
  >
</div>

<style>
  div {
    display: flex;
    flex-direction: column;
  }
  label {
    display: flex;
    background-color: var(--bg-color);
    margin-top: 1rem;
  }
  a {
    display: flex;
    color: var(--text-color);
    position: relative;
    margin-top: 1rem;
  }
  button {
    background-color: var(--bg-color);
    color: var(--text-color);
    margin-top: 1rem;
  }
  button a {
    margin: auto;
  }
</style>
