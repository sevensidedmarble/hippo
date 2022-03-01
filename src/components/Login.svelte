<script lang="ts">
  let email, password

  interface LoginData {
    email: string,
    password: string
  }

  async function login(data: LoginData) {
    const opts = {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    }
    const res = await fetch('http://localhost:8888/auth/login', opts)
    const body = await res.json()
    console.info('Auth response:', body)
  }

  async function submit(e) {
    console.info('Attempting login.')

    await login({ email, password })
  }
</script>

<form on:submit|preventDefault={submit} aria-label="login" class="w-96 mx-auto md:m-6 md:mx-10 lg:m12 lg:mx-20">
  <h1 class="text-4xl font-bold mb-6">Login now!</h1>
  <div class="form-control">
    <label class="label">
      <span class="label-text">Email</span>
    </label>
    <input bind:value={email} type="text" placeholder="Email" class="input input-bordered">
  </div>
  <div class="form-control">
    <label class="label">
      <span class="label-text">Password</span>
    </label>
    <input bind:value={password} type="text" placeholder="Password" class="input input-bordered">
    <label class="label">
      <a href="#" class="label-text-alt link link-hover">Forgot password?</a>
    </label>
  </div>
  <div class="mt-6">
    <button type="submit" class="w-full mx-auto btn btn-primary">Login</button>
  </div>
</form>

<style>
  .label-text {
    @apply text-lg font-semibold;
  }

  .form-control {
    @apply mt-3;
  }
</style>
