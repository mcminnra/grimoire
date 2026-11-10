import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import pandacss from "@pandacss/vite";
import process from "node:process";
const host = process.env.TAURI_DEV_HOST;

// https://vite.dev/config/
export default defineConfig(() => ({
  plugins: [sveltekit(), pandacss()],
  clearScreen: false, // prevent Vite from obscuring rust errors
  server: {
    port: 1420, // tauri expects a fixed port, fail if that port is not available
    strictPort: true,
    host: host || "127.0.0.1",
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    fs: {
      allow: ["generated/styled-system"],
    },
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
