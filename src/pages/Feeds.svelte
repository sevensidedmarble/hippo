<script lang="ts">
  import Counter from '../components/Counter.svelte'
  import { timeAgo } from '../utils'
  import { feeds } from '../stores/feeds'

  function getRandomInt(max) {
    return Math.floor(Math.random() * max);
  }
</script>

<section>
  <!-- Page header section -->
  <div class="flex">
    <h2 class="flex-grow font-bold text-2xl text-gray-900 py-2 my-2">My Feeds</h2>
    <button class="flex-shrink-0 text-white bg-indigo-500 border-0 focus:outline-indigo-700 hover:bg-indigo-600 rounded font-bold text-lg m-2 p-3">Add Feed</button>
  </div>

  <!-- Feeds list -->
  {#each $feeds as feed}
    <li class="flex mt-8 gap-6">
      <!-- Title -->
      <div class="flex flex-col flex-auto">
        <div class="font-bold text-lg">{feed.title}</div>
        <div class="text-gray-600 ">{feed.description}</div>
      </div>
      <!-- Recent Info -->
      <div class="text-gray-600 text-md inline-flex items-center">
        <div class="flex flex-col">
          <div class="text-white text-xs inline-flex items-center font-bold self-end">
            <div class="text-center bg-accent rounded-full py-1 px-2 my-1">
              {getRandomInt(5)+1} new posts
            </div>
          </div>
          <div class="text-gray-600 text-sm">Fetched {timeAgo(new Date(feed.fetched_at + "Z"))}</div>
        </div>
      </div>
      <!-- Menu -->
      <div class="inline-flex items-center">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 5v.01M12 12v.01M12 19v.01M12 6a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2zm0 7a1 1 0 110-2 1 1 0 010 2z" />
        </svg>
      </div>
    </li>
  {/each}
</section>
