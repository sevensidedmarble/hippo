import { defineConfig, loadEnv } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import analyze from 'rollup-plugin-analyzer'
import viteCompression from 'vite-plugin-compression'

export default ({ mode }) => {
  process.env = { ...process.env, ...loadEnv(mode, process.cwd(), '') }

  const host = process.env.DEV_SERVER_HOST
  const port = process.env.DEV_SERVER_PORT

  return defineConfig({
    build: {
      sourcemap: true
    },
    plugins: [
      svelte(),
      viteCompression({
        algorithm: 'gzip'
      }),
      viteCompression({
        algorithm: 'brotliCompress'
      }),
      analyze()
    ],
    server: {
      host,
      port
    }
  })
}
