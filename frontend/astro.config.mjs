// @ts-check
import { defineConfig } from "astro/config";

import svelte from "@astrojs/svelte";
import I18nIntegration from "src/i18n-integration";

// https://astro.build/config
export default defineConfig({
  integrations: [
    I18nIntegration(),
    svelte(),
    (await import("@playform/compress")).default(),
  ],
});
