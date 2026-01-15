import { defineConfig } from "vite";

// https://vitejs.dev/config/
export default defineConfig({
  // Prevent vite from obscuring rust errors
  clearScreen: false,
  server: {
    // Tauri expects a fixed port, fail if that port is not available
    port: 1420,
    strictPort: true,
    // Allow Tauri to connect
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
});
