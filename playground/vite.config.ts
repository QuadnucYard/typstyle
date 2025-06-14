import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import solid from "vite-plugin-solid";
import toplevelAwait from "vite-plugin-top-level-await";
import tailwindcss from "@tailwindcss/vite";
import AutoImport from "unplugin-auto-import/vite";

// https://vite.dev/config/
export default defineConfig({
  base: "/typstyle/playground/",

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
      output: {
        manualChunks: (id): string | undefined => {
          // Large packages get their own chunks
          if (id.includes("monaco-editor")) {
            return "monaco-editor";
          }
          if (id.includes("monaco-themes")) {
            return "monaco-themes";
          }
          if (id.includes("react-dom")) {
            return "react-dom";
          }
          if (id.includes("react")) {
            return "react";
          }

          // Group all application source code and public resources together
          if (id.includes("/src/")) {
            return "app";
          }
          // NOTE: If we pack some scripts together, it may raise loading error in production.

          // // Default chunk for everything else
          // return "vendor";
        },
      },
    },
  },
});
