<script lang="ts">
	import CopyButton from './CopyButton.svelte';

	const installCommand = 'curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash';
</script>

<section id="hero" class="hero">
	<div class="container">
		<h1 class="hero-title">
			<div class="title-row">
				<span class="stomp-word stomp-1">100%</span>
			</div>
			<div class="title-row">
				<span class="stomp-word stomp-2">Local.</span>
				<span class="stomp-word stomp-3">Private.</span>
				<span class="stomp-word stomp-4">Yours.</span>
			</div>
		</h1>
		<p class="hero-subtitle">AI-powered CLI command generator designed to run <strong>completely locally</strong> using Ollama</p>
		<p class="hero-description">No data leaves your machine. No telemetry. No cloud dependencies. Just you, your terminal, and AI-powered command generation that respects your privacy.</p>

		<div class="install-command">
			<code>{installCommand}</code>
			<CopyButton text={installCommand} inline={false} />
		</div>

		<div class="hero-links">
			<a href="https://github.com/ezcorp-org/ez-term" class="btn btn-primary" target="_blank" rel="noopener noreferrer">
				⭐ Star on GitHub
			</a>
			<a href="https://github.com/ezcorp-org/ez-term#-quick-start" class="btn btn-secondary" target="_blank" rel="noopener noreferrer">
				🚀 Quick Start
			</a>
		</div>
	</div>
</section>

<style>
	.hero {
		padding: var(--space-3xl) 0;
		text-align: center;
	}

	.hero-title {
		font-size: clamp(2.5rem, 5vw, 4rem);
		margin-bottom: var(--space-md);
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.3em;
		animation:
			screenShake 0.4s ease-out 0.7s,
			screenShake 0.4s ease-out 1.5s,
			screenShake 0.4s ease-out 1.9s,
			screenShake 0.4s ease-out 2.3s;
	}

	.title-row {
		display: flex;
		gap: clamp(1rem, 3vw, 2rem);
		justify-content: center;
		align-items: center;
	}

	.stomp-word {
		display: inline-block;
		background: linear-gradient(135deg, var(--primary), var(--success));
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		opacity: 0;
		animation: smoothFallIn 1s cubic-bezier(0.34, 1.56, 0.64, 1) forwards;
	}

	/* Staggered delays - 100% drops first, then words */
	.stomp-1 {
		animation-delay: 0s;
		font-size: 1.2em;
		font-weight: 800;
	}

	.stomp-2 {
		animation-delay: 0.8s;
	}

	.stomp-3 {
		animation-delay: 1.2s;
	}

	.stomp-4 {
		animation-delay: 1.6s;
	}

	/* Smooth fall-in animation with bounce */
	@keyframes smoothFallIn {
		0% {
			opacity: 0;
			transform: translateY(-150px) scale(1.5);
		}
		50% {
			opacity: 1;
			transform: translateY(-20px) scale(1.1);
		}
		70% {
			transform: translateY(10px) scale(0.98);
		}
		85% {
			transform: translateY(-5px) scale(1.01);
		}
		100% {
			opacity: 1;
			transform: translateY(0) scale(1);
		}
	}

	/* Screen shake effect on each impact */
	@keyframes screenShake {
		0%, 100% {
			transform: translate(0, 0);
		}
		10%, 30%, 50%, 70%, 90% {
			transform: translate(-3px, 3px);
		}
		20%, 40%, 60%, 80% {
			transform: translate(3px, -3px);
		}
	}

	/* Reduce motion for accessibility */
	@media (prefers-reduced-motion: reduce) {
		.stomp-word {
			animation: simpleFadeIn 0.5s ease-out forwards;
		}

		.stomp-1 {
			animation-delay: 0s;
		}

		.stomp-2 {
			animation-delay: 0.2s;
		}

		.stomp-3 {
			animation-delay: 0.4s;
		}

		.stomp-4 {
			animation-delay: 0.6s;
		}

		.hero-title {
			animation: none;
		}

		@keyframes simpleFadeIn {
			from {
				opacity: 0;
			}
			to {
				opacity: 1;
			}
		}
	}

	.hero-subtitle {
		font-size: clamp(1.2rem, 2.5vw, 1.5rem);
		color: var(--text-primary);
		margin-bottom: var(--space-md);
		max-width: 800px;
		margin-left: auto;
		margin-right: auto;
	}

	.hero-description {
		font-size: 1.1rem;
		color: var(--text-secondary);
		max-width: 700px;
		margin: 0 auto var(--space-xl);
		line-height: 1.8;
	}

	.install-command {
		position: relative;
		max-width: 700px;
		margin: 0 auto var(--space-xl);
		background-color: var(--bg-elevated);
		border: 1px solid var(--border);
		border-radius: var(--border-radius);
		padding: var(--space-md) var(--space-lg);
		display: flex;
		align-items: center;
		gap: var(--space-md);
		justify-content: space-between;
	}

	.install-command code {
		font-family: var(--font-mono);
		font-size: 0.95rem;
		color: var(--success);
		background: none;
		padding: 0;
		flex: 1;
		text-align: left;
		word-break: break-all;
	}

	.hero-links {
		display: flex;
		gap: var(--space-md);
		justify-content: center;
		flex-wrap: wrap;
	}

	@media (max-width: 768px) {
		.hero {
			padding: var(--space-xl) 0;
		}

		.install-command {
			flex-direction: column;
			align-items: stretch;
		}

		.install-command code {
			text-align: center;
			word-break: break-word;
		}
	}
</style>
