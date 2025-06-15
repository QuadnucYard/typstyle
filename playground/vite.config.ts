import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import solid from "vite-plugin-solid";
import toplevelAwait from "vite-plugin-top-level-await";
import tailwindcss from "@tailwindcss/vite";
import AutoImport from "unplugin-auto-import/vite";

// https://vite.dev/config/
export default defineConfig({
  base: "/typstyle/playground/",

  optimizeDeps: {
    exclude: ["monaco-editor"],
  },

  plugins: [
    solid(),
    tailwindcss(),
    wasm(),
    toplevelAwait(), // required by wasm

    AutoImport({
      imports: ["solid-js"],
      dts: "src/auto-imports.d.ts",
    }),
  ],

  build: {
    rollupOptions: {
      external: ["monaco-editor"],
      output: {
        manualChunks: (id): string | undefined => {
          // Group all application source code and public resources together
          if (id.includes("/src/")) {
            return "app";
          }
        },
      },
    },
  },
});
