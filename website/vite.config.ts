import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		host: '0.0.0.0',
		allowedHosts: [
			'nixos.taile1c5b0.ts.net',
			'.ts.net'
		]
	},
	preview: {
		host: '0.0.0.0',
		allowedHosts: [
			'nixos.taile1c5b0.ts.net',
			'.ts.net'
		]
	}
});
