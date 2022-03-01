<script lang="ts">
  import Modal from './Modal.svelte'
  import Login from './Login.svelte'

  let modal
  export const showModal = () => {
    modal.showModal()
  }
</script>

<Modal bind:this={modal} component={Login}></Modal>
