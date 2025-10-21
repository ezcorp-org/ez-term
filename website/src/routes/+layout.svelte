<script lang="ts">
	import { base } from '$app/paths';
	import SEO from '$lib/components/SEO.svelte';
	import Header from '$lib/components/Header.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import '$lib/styles/global.css';
	import '$lib/styles/terminal.css';

	let { children } = $props();
</script>

<SEO />

<svelte:head>
	<link rel="icon" type="image/svg+xml" href="{base}/assets/images/favicon.svg" />
	<link rel="apple-touch-icon" href="{base}/assets/images/favicon.svg" />
</svelte:head>

<Header />
<main>
	{@render children?.()}
</main>
<Footer />
