<script lang="ts">
  import { posts } from '../stores/posts'
  import { timeAgo } from '../utils'
</script>

<section>
  <!-- Page header section -->
  <div class="flex">
    <h2 class="flex-grow font-bold text-2xl text-gray-900 py-2 my-2">Unread</h2>
    <button class="flex-shrink-0 text-white bg-indigo-500 border-0 focus:outline-indigo-700 hover:bg-indigo-600 rounded font-bold text-lg m-2 p-3">Add Feed</button>
  </div>

  {#each $posts as post}
    <li class="list-none">
      <a class="focus:bg-gray-100 hover:bg-gray-100 flex mt-16 gap-6 p-2 rounded" href="#">
        <!-- Published on -->
        <div class="text-gray-600 text-sm text-center">Published {timeAgo(new Date(post.published_at + "Z"))}</div>
        <!-- Title -->
        <div class="flex-grow flex flex-col mb-4">
          <div class="hover:underline hover:text-blue-400 font-bold text-2xl mb-6">{post.title}</div>
          <div class="text-gray-600 max-h-6 overflow-hidden text-ellipsis">{post.summary}</div>
        </div>
      </a>
    </li>
  {/each}
</section>
