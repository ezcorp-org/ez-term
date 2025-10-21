<script lang="ts">
	import { softwareApplicationSchema } from '$lib/utils/schema';

	interface Props {
		title?: string;
		description?: string;
		keywords?: string;
		ogImage?: string;
		canonicalUrl?: string;
	}

	let {
		title = "ez-term - Privacy-First AI CLI Command Generator | 100% Local with Ollama",
		description = "Generate CLI commands with AI that runs 100% locally on your machine. Privacy-first, zero telemetry, context-aware command generation with Ollama. Free and open source.",
		keywords = "CLI command generator, AI terminal assistant, Ollama, local AI, privacy-first, command line tool, terminal automation, shell commands, bash generator, developer tools, open source CLI",
		ogImage = "https://ezcorp-org.github.io/ez-term/assets/images/og-preview.png",
		canonicalUrl = "https://ezcorp-org.github.io/ez-term/"
	}: Props = $props();

	const schemaJSON = JSON.stringify(softwareApplicationSchema);
</script>

<svelte:head>
	<!-- Primary Meta Tags -->
	<title>{title}</title>
	<meta name="title" content={title} />
	<meta name="description" content={description} />
	<meta name="keywords" content={keywords} />
	<meta name="author" content="EZ Corp" />
	<meta name="robots" content="index, follow" />
	<link rel="canonical" href={canonicalUrl} />

	<!-- Open Graph / Facebook -->
	<meta property="og:type" content="website" />
	<meta property="og:url" content={canonicalUrl} />
	<meta property="og:site_name" content="ez-term - Privacy-First AI CLI Tool" />
	<meta property="og:title" content="ez-term - 100% Local AI CLI Command Generator" />
	<meta property="og:description" content="Privacy-first CLI command generator that runs entirely on your machine with Ollama. No data leaves your computer. Zero telemetry. Context-aware and safe by default." />
	<meta property="og:image" content={ogImage} />
	<meta property="og:image:width" content="1200" />
	<meta property="og:image:height" content="630" />
	<meta property="og:image:alt" content="ez-term - Privacy-First AI CLI Command Generator" />
	<meta property="og:locale" content="en_US" />

	<!-- Twitter Card -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:url" content={canonicalUrl} />
	<meta name="twitter:title" content="ez-term - 100% Local AI CLI Commands" />
	<meta name="twitter:description" content="Privacy-first CLI command generator. Runs completely locally with Ollama. No telemetry. No cloud dependencies. Free and open source." />
	<meta name="twitter:image" content={ogImage} />
	<meta name="twitter:image:alt" content="ez-term terminal showing AI-generated command" />
	<meta name="twitter:creator" content="@EZCorp" />

	<!-- Additional Meta Tags for SEO -->
	<meta name="theme-color" content="#0D1117" />
	<meta name="apple-mobile-web-app-capable" content="yes" />
	<meta name="apple-mobile-web-app-status-bar-style" content="black-translucent" />
	<meta name="apple-mobile-web-app-title" content="ez-term - AI CLI" />

	<!-- Structured Data / Schema.org -->
	{@html `<script type="application/ld+json">${schemaJSON}<\/script>`}
</svelte:head>
