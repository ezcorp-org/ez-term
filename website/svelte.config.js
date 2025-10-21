import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: null,
			precompress: false,
			strict: true
		}),
		paths: {
			base: process.env.GITHUB_PAGES === 'true' ? '/ez-term' : ''
		},
		prerender: {
			handleMissingId: 'warn',
			handleHttpError: 'warn'
		}
	}
};

export default config;
