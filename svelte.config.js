import adapter from "@sveltejs/adapter-static";
import { vitePreprocess } from "@sveltejs/vite-plugin-svelte";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: vitePreprocess(),
  kit: {
    adapter: adapter({
      pages: "dist/web",
      assets: "dist/web",
      fallback: "index.html",
    }),
    alias: {
      "styled-system": "./generated/styled-system/*",
    },
  },
};

export default config;
