<script lang="ts">
  import { posts } from '../stores/posts'
  import { timeAgo, capitalizeFirst } from '../utils'
  import { parseJSON, formatRelative } from 'date-fns'

  import Dropdown from '../components/Dropdown.svelte'
</script>

<section>
  <!-- Page header section -->
  <div class="flex gap-2 my-6 px-4">
    <h2 class="flex-grow font-bold text-2xl leading-10 text-gray-900">Unread</h2>
    <Dropdown></Dropdown>
    <div>
      <button class="flex-shrink-0 text-white bg-primary border-0 focus:outline-indigo-700 hover:bg-indigo-600 rounded-xl font-bold p-2 px-4">Add Feed</button>
    </div>
  </div>

  <ul>
  {#each $posts as post}
    <li class="list-none">
      <a class="summary hover:max-h-max focus-within:max-h-max max-h-36 focus:bg-gray-100 hover:bg-gray-100 flex flex-col mt-16 gap-6 p-4 rounded" target="_blank" rel="noopener noreferrer" href={post.url}>
        <!-- Title -->
        <div class="flex-grow flex flex-col mb-4">
          <div class="hover:underline hover:text-blue-400 font-bold text-2xl mb-2">{post.title}</div>

          <!-- Published on -->
          {#if post.published_at}
            <div class="text-gray-600 text-xs mb-5">{capitalizeFirst(formatRelative(parseJSON(post.published_at), new Date()))}</div>
          {/if}

          <div class="overflow-hidden prose-sm">{@html post.summary}</div>
        </div>
      </a>
    </li>
  {/each}
  </ul>
</section>

<style>
  .summary {
    /* mask-image: linear-gradient(to bottom, black 50%, transparent 100%); */
    overflow: hidden;
  }

  .summary:hover, .summary:focus-within, .summary:focus {
    mask-image: initial;
  }
</style>
