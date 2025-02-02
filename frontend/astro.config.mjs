// @ts-check
import { defineConfig } from "astro/config";

import svelte from "@astrojs/svelte";

import tailwindcss from "@tailwindcss/vite";

// https://astro.build/config
export default defineConfig({
  integrations: [
    svelte(),
    (await import("@playform/compress")).default(),
  ],
  vite: {
    // @ts-ignore
    plugins: [tailwindcss()], 
    server: {
      fs: {
        allow: [
          ".",
          "../cbf" // Allow reading from the cbf directory for loading the WebAssembly module
        ]
      }
    }
  }
});
