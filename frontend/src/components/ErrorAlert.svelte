<!-- svelte-ignore options_missing_custom_element -->
<svelte:options
  customElement={{
    tag: "error-alert",
    shadow: "none",
  }}
/>

<script lang="ts">
  import { isErrorVisible } from "./stores";
  import { onMount } from "svelte";
  let { message }: { message: string } = $props();

  let alertContainer: HTMLElement;

  onMount(() => {
    alertContainer = document.getElementById("alert") as HTMLElement;
  });
</script>

<div class="alert-container" id="alert">
  <div role="alert" class="alert alert-error">
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6 shrink-0 stroke-current"
      fill="none"
      viewBox="0 0 24 24"
      onclick={() => {
        alertContainer.remove();
        $isErrorVisible = false;
      }}
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
      />
    </svg>
    <span>{message}</span>
  </div>
</div>

<style>
  .alert-container {
    margin-top: 2rem;
    position: fixed;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: auto;
    max-width: 90%;
    z-index: 1000;
    animation: slide-down 0.5s ease-out;
  }

  @media only screen and (max-width: 600px) {
    .alert-container {
      width: 100%;
    }
    .alert {
      display: flex;
      align-items: center;
      justify-content: center;
    }
  }

  @keyframes slide-down {
    from {
      transform: translateX(-50%) translateY(-100%);
    }
    to {
      transform: translateX(-50%) translateY(0);
    }
  }
  svg {
    cursor: pointer;
  }
</style>
