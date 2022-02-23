<script lang="ts">
  import { onMount } from 'svelte'
  import { user } from './stores/auth'
  import { feeds } from './stores/feeds'
  import { posts } from './stores/posts'

  // Routes
  import { Route } from 'tinro'
  import Feeds from './pages/Feeds.svelte'
  import Home from './pages/Home.svelte'

  async function getFeeds () {
    const opts = {
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
    }
    const res = await fetch('http://localhost:8888/users/1/feeds', opts)
    const data = await res.json()
    console.log(data.feeds)
    return data.feeds
  }

  async function getPosts () {
    const opts = {
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
    }
    const res = await fetch('http://localhost:8888/users/1/posts', opts)
    const data = await res.json()
    console.log(data.posts)
    return data.posts
  }

  onMount(async () => {
    $feeds = await getFeeds()
    $posts = await getPosts()
  })

  async function login() {
    const data = {
      email: 'hello',
      password: 'world'
    }
    const opts = {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    }
    const res = await fetch('http://localhost:8888/auth/login', opts)
    let u = await res.json()
  }

</script>

<div class="flex flex-col">
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
      <a class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-1" href="#">
        Posts
      </a> 
      <a class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-1" href="#">
        Account
      </a>
    </nav>
    <div class="">
      {#if $user === null}
        <a class="font-bold text-gray-500 focus:text-base-content hover:text-base-content transition-colors p-1 px-4" href="#">
          Login
        </a>
        <button class="bg-primary hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-xl" on:click={login}>
          Register
        </button>
      {:else}
        <button class="btn btn-ghost">
          Account
        </button>
      {/if}
    </div>
  </div>
  <main class="container md:max-w-screen-lg mx-auto px-4 mt-10">
    <Route path="/"><Home /></Route>
    <Route path="/feeds"><Feeds /></Route>
  </main>
</div>
