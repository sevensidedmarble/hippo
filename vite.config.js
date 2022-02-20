import { defineConfig, loadEnv } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default ({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd(), '') }

  const host = process.env.DEV_SERVER_HOST
  const port = process.env.DEV_SERVER_PORT

  return defineConfig({
    plugins: [svelte()],
    server: {
      host,
      port
    }
  })
}
