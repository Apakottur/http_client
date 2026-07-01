import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'

// Tauri desktop dev config. See https://v2.tauri.app and https://vite.dev/config/
export default defineConfig({
  plugins: [svelte()],

  // Keep Rust compiler output visible in the terminal (Vite clears the screen by default).
  clearScreen: false,

  server: {
    // Tauri's devUrl points at :5173 — fail loudly if it's taken instead of silently
    // switching to another port that Tauri wouldn't find.
    port: 5173,
    strictPort: true,
    // Tauri watches and rebuilds the Rust side itself. If Vite also watched src-tauri/,
    // the huge file churn under target/ during each cargo rebuild would flood the HMR
    // watcher and disrupt live reload. Ignore it here.
    watch: { ignored: ['**/src-tauri/**'] },
  },
})
