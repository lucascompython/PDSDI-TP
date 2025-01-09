/** @type {import('tailwindcss').Config} */
import daisyui from "daisyui";
export default {
  content: ["./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}"],
  theme: {
    extend: {
      // Remove custom background colors
      backgroundColor: false,
    },
  },
  daisyui: {
    // Remove DaisyUI themes
    themes: [],
  },

  plugins: [daisyui],
  corePlugins: {
    backgroundColor: false, // Disable background color utilities
  },
};
