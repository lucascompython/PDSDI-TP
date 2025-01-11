<!-- svelte-ignore options_missing_custom_element -->
<svelte:options
  customElement={{
    tag: "alert-popup",
    shadow: "none",
  }}
/>

<script lang="ts">
  import { isErrorVisible } from "../stores";
  import { AlertType } from "./Alert";
  let { message, type }: { message: string; type: AlertType } = $props();

  let alertClass: string = $state("");
  let svgData: string = $state("");

  switch (type) {
    case AlertType.ERROR:
      alertClass = "alert-error";
      svgData =
        "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";
      break;
    case AlertType.SUCCESS:
      alertClass = "alert-success";
      svgData = "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z";
      break;
    case AlertType.WARNING:
      alertClass = "alert-warning";
      svgData =
        "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z";
      break;
    default:
      alertClass = "alert-error";
      svgData =
        "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";
      break;
  }
</script>

<button
  class="alert-container"
  id="alert"
  onclick={(e) => {
    e.currentTarget.remove();

    $isErrorVisible = false;
  }}
>
  <div role="alert" class="alert {alertClass}">
    <svg
      xmlns="http://www.w3.org/2000/svg"
      class="h-6 w-6 shrink-0 stroke-current"
      fill="none"
      viewBox="0 0 24 24"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        stroke-width="2"
        d={svgData}
      />
    </svg>
    <span>{message}</span>
  </div>
</button>

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
</style>
