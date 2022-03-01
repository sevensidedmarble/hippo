<script lang="ts">
  export let open = false

  const show = () => {
    if (!open) { open = true }
  }

  const hide = () => {
    if (open && document.hasFocus()) { open = false }
  }
</script>

<div class="relative inline-flex place-items-center text-left">
  <div>
    <button on:focus={show} on:blur={hide} type="button" class="inline-flex justify-center w-full rounded-md px-4 pr-6 py-2 text-md font-semibold text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-gray-100 focus:ring-indigo-500" id="menu-button" aria-expanded="true" aria-haspopup="true">
      Filter
      <!-- Heroicon name: solid/chevron-down -->
      <svg class="-mr-4 ml-1 mr-.5 h-6 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
        <path fill-rule="evenodd" d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clip-rule="evenodd" />
      </svg>
    </button>
  </div>

  <div class:hide-visual={!open} class="origin-top-right absolute right-0 top-12 mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none" role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1">
    <div class="py-1" role="none">
      <a on:focus={show} on:blur={hide} href="#" class="menu-item" role="menuitem">Unread</a>
      <a on:focus={show} on:blur={hide} href="#" class="menu-item" role="menuitem">Today</a>
      <a on:focus={show} on:blur={hide} href="#" class="menu-item" role="menuitem">All</a>
    </div>
  </div>
</div>

<style>
  .menu-item {
    @apply hover:bg-gray-100 hover:text-gray-900 text-gray-700 block px-4 py-2 text-sm
  }

  /* Hide visually without effecting tab order */
  .hide-visual:not(:focus):not(:active) {
    position: absolute;

    width: 1px;
    height: 1px;
    margin: -1px;
    border: 0;
    padding: 0;

    white-space: nowrap;

    clip-path: inset(100%);
    clip: rect(0 0 0 0);
    overflow: hidden;
}
</style>
