import { defineConfig } from "vite";

export default defineConfig({
  root: "frontend",
  server: {
    port: 3000,
    open: true,
  },
  build: {
    target: "esnext",
    outDir: "../dist",
    emptyOutDir: true,
  },
  optimizeDeps: {
    exclude: ["@vite/client", "@vite/env"],
  },
});
