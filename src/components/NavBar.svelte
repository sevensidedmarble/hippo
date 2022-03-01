<script>
  import { user } from '../stores/auth'
  import LoginModal from '../components/LoginModal.svelte'

  let modal
</script>

<LoginModal bind:this={modal}></LoginModal>

<div class="flex top-0 bg-opacity-60 text-center m-8 mt-6 pb-2">
  <div class="flex items-center">
    <div class="sm:px-2 mx-2 text-xl font-bold select-none">Hippo 🦛</div>
  </div> 
  <nav class="flex px-2 mx-2 x-2 flex-wrap flex-grow items-center place-content-center gap-4">
    <a class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-1" href="/">
      Home
    </a> 
    <a class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-1" href="/feeds">
      Feeds
    </a> 
    <a class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-1" href="/">
      Posts
    </a> 
    <a class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-1" href="/">
      Account
    </a>
  </nav>
  <div class="">
    {#if $user === null}
      <button class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-2 px-4 mr-3" on:click={() => modal.showModal()}>
        Login
      </button>
      <button class="bg-primary hover:bg-blue-700 text-white font-bold p-2 py-2 px-4 rounded-xl">
        Register
      </button>
    {:else}
      <button class="btn btn-ghost">
        Account
      </button>
    {/if}
  </div>
</div>
