import { defineConfig } from "vite";

// Tauri looks for a dev server on this port and injects the binary with CSP.
// Keep it in sync with `devUrl` in src-tauri/tauri.conf.json.
export default defineConfig({
  base: "./",
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: "127.0.0.1",
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: "es2021",
    minify: "esbuild",
    sourcemap: false,
  },
});
