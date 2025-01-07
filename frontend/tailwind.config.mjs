/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		extend: {
			// Remove custom background colors
			backgroundColor: false,
		},
	},
	
	plugins: [
		require('daisyui'),
	],
	corePlugins: {
		backgroundColor: false, // Disable background color utilities
	},
}
