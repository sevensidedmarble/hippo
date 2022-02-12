<script lang="ts">
  import Counter from './lib/Counter.svelte'
</script>

<div class="flex flex-col">
  <div class="navbar sticky top-0 w-full border-b border-gray-200 bg-clip-padding backdrop-filter backdrop-blur bg-opacity-60 mb-2 pb-2">
    <div class="px-2 mx-2 navbar-start">
      <span class="text-xl font-bold">
        <span class="text-accent">Every</span><span class="font-medium">link</span>
      </span>
    </div> 
    <div class="hidden px-2 mx-2 navbar-center lg:flex">
      <div class="flex items-stretch">
        <a class="btn btn-ghost btn-sm rounded-btn">
          Home
        </a> 
        <a class="btn btn-ghost btn-sm rounded-btn">
          Extension
        </a> 
        <a class="btn btn-ghost btn-sm rounded-btn">
          Pricing
        </a> 
        <a class="btn btn-ghost btn-sm rounded-btn">
          Admin
        </a>
      </div>
    </div> 
    <div class="navbar-end">
      <button class="btn btn-ghost">
        Add to Chrome
      </button>
    </div>
  </div>
  <main>
    <section class="text-gray-600 body-font">
      <div class="container px-5 py-24 mx-auto">
        <div class="lg:w-2/3 flex flex-col sm:flex-row sm:items-center items-start mx-auto">
          <h1 class="flex-grow sm:pr-16 text-2xl font-medium title-font text-gray-900">Slow-carb next level shoindxgoitch ethical authentic, scenester sriracha forage.</h1>
          <button class="flex-shrink-0 text-white bg-indigo-500 border-0 py-2 px-8 focus:outline-none hover:bg-indigo-600 rounded text-lg mt-10 sm:mt-0">Add to Chrome</button>
        </div>
      </div>
    </section>
  </main>
</div>

<style>
</style>
