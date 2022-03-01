<script lang="ts">
  import { timeAgo } from '../utils'
  import { feeds } from '../stores/feeds'
  import { parseJSON, formatRelative } from 'date-fns'

  function getRandomInt(max) {
    return Math.floor(Math.random() * max);
  }
</script>

<section>
  <!-- Page header section -->
  <div class="flex gap-2 my-6 px-4">
    <h2 class="flex-grow font-bold text-2xl leading-10 text-gray-900">My Feeds</h2>
    <div>
      <button class="flex-shrink-0 text-white bg-primary border-0 focus:outline-indigo-700 hover:bg-indigo-600 rounded-xl font-bold p-2 px-4">Add Feed</button>
    </div>
  </div>

  <!-- Feeds list -->
  <ul>
  {#each $feeds as feed}
    <li class="flex mt-12 gap-6 p-4 py-1">
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
          <div class="text-gray-600 text-sm">{formatRelative(parseJSON(feed.fetched_at), new Date())}</div>
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
  </ul>
</section>
