import { defineConfig } from "vite";
// PLUGINS
import wasm from "vite-plugin-wasm";
import wasmPack from "vite-plugin-wasm-pack";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [wasm(), wasmPack("./wasm"), topLevelAwait(), svelte()],
});
