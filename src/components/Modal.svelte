<script lang="ts">
  import { writable } from 'svelte/store'
  import SimpleModal, { bind } from 'svelte-simple-modal'

  export let component

  const modal = writable(null)
  export const showModal = () => modal.set(component)
</script>

<SimpleModal 
  show={$modal}
  classBg="fixed top-0 left-0 w-screen h-full flex align-content-stretch items-end sm:items-center place-content-center bg-neutral-focus bg-opacity-40 "
  classWindowWrap=""
  classWindow="w-screen min-h-10 sm:w-auto sm:h-auto translate-y-0 transform bg-base-100 p-6 transition duration-200 ease-in-out rounded-t-box sm:translate-y-0 sm:scale-90 sm:rounded-b-box"
  classContent=""
  unstyled={true}>
</SimpleModal>

<style>
  .modal {
    @apply bg-neutral-focus bg-opacity-40 duration-200 ease-in-out;
    transition-property: transform, opacity;
    overflow-y: hidden;
    overscroll-behavior: contain;
  }
  .modal-box {
    @apply w-full translate-y-10 transform bg-base-100 p-6 transition duration-200 ease-in-out rounded-t-box sm:max-w-lg sm:translate-y-0 sm:scale-90 sm:rounded-b-box;
    box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.25);
    overflow-y: auto;
    overscroll-behavior: contain;
  }
  .modal-open .modal-box,
  .modal-toggle:checked + .modal .modal-box,
  .modal:target .modal-box {
    @apply translate-y-0 scale-100;
  }
  .modal-action {
    @apply mt-6 justify-end space-x-2;
  }
</style>
