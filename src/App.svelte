<script lang="ts">
  import { onMount } from 'svelte'
  import { feeds } from './stores/feeds'
  import { posts } from './stores/posts'

  // components
  import NavBar from './components/NavBar.svelte'

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

</script>

<div class="flex flex-col">
  <NavBar></NavBar>
  <main class="container md:max-w-screen-lg mx-auto px-4 mt-10">
    <Route path="/"><Home /></Route>
    <Route path="/feeds"><Feeds /></Route>
  </main>
</div>
