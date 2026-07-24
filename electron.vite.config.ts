import { resolve } from 'path'
import { defineConfig } from 'electron-vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import pkg from './package.json'

export default defineConfig({
  main: {},
  preload: {},
  renderer: {
    define: {
      __APP_VERSION__: JSON.stringify(pkg.version)
    },
    resolve: {
      alias: {
        '@lib': resolve('src/renderer/src/lib'),
        '@components': resolve('src/renderer/src/components'),
        '@views': resolve('src/renderer/src/views')
      }
    },
    plugins: [svelte()]
  }
})
