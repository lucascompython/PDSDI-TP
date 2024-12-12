<script>
  import DropdownIcon from "./DropdownIcon.svelte";

  let isDropdownOpen = false; // default state (dropdown close)

  const handleDropdownClick = () => {
    isDropdownOpen = !isDropdownOpen; // togle state on click
  };

  const handleDropdownFocusLoss = ({ relatedTarget, currentTarget }) => {
    // use "focusout" event to ensure that we can close the dropdown when clicking outside or when we leave the dropdown with the "Tab" button
    if (
      relatedTarget instanceof HTMLElement &&
      currentTarget.contains(relatedTarget)
    )
      return; // check if the new focus target doesn't present in the dropdown tree (exclude ul\li padding area because relatedTarget, in this case, will be null)
    isDropdownOpen = false;
  };
</script>

<div class="dropdown" on:focusout={handleDropdownFocusLoss}>
  <button on:click={handleDropdownClick}>
    <DropdownIcon />
  </button>
  <ul
    class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52"
    style:visibility={isDropdownOpen ? "visible" : "hidden"}
  >
    <li><button class="btn text-slate-300">Item 1</button></li>
    <li><button class="btn text-slate-300">Item 2</button></li>
  </ul>
</div>

<style>
  .dropdown {
    display: flex;
    justify-content: center;
    align-items: center;
  }
</style>
