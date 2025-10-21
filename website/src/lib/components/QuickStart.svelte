<script lang="ts">
	import CopyButton from './CopyButton.svelte';
</script>

<section id="quick-start" class="quick-start">
	<div class="container">
		<h2 class="section-title">Quick Start</h2>
		<div class="quick-start-content">
			<div class="quick-start-step">
				<h3>1. Install ez-term</h3>
				<div class="code-block">
					<code>curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash</code>
					<CopyButton text="curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash" inline={true} />
				</div>
			</div>
			<div class="quick-start-step">
				<h3>2. Install Ollama</h3>
				<p>Visit <a href="https://ollama.ai" target="_blank" rel="noopener noreferrer">ollama.ai</a> for installation instructions</p>
				<div class="code-block">
					<code>ollama pull qwen2.5-coder:latest</code>
					<CopyButton text="ollama pull qwen2.5-coder:latest" inline={true} />
				</div>
			</div>
			<div class="quick-start-step">
				<h3>3. Configure ez-term</h3>
				<div class="code-block">
					<code>ez --set-backend ollama{'\n'}ez --set-model qwen2.5-coder:latest</code>
					<CopyButton text="ez --set-backend ollama\nez --set-model qwen2.5-coder:latest" inline={true} />
				</div>
			</div>
			<div class="quick-start-step">
				<h3>4. Start using it!</h3>
				<div class="code-block">
					<code>ez "your question here"</code>
					<CopyButton text='ez "your question here"' inline={true} />
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.quick-start {
		padding: var(--space-2xl) 0;
		background-color: var(--bg-darker);
	}

	.section-title {
		text-align: center;
		margin-bottom: var(--space-xl);
		color: var(--text-primary);
	}

	.quick-start-content {
		max-width: 800px;
		margin: 0 auto;
		display: flex;
		flex-direction: column;
		gap: var(--space-xl);
	}

	.quick-start-step {
		background-color: var(--bg-elevated);
		padding: var(--space-xl);
		border-radius: var(--border-radius);
		border: 1px solid var(--border);
	}

	.quick-start-step h3 {
		color: var(--text-primary);
		margin-bottom: var(--space-md);
	}

	.quick-start-step p {
		color: var(--text-secondary);
		margin-bottom: var(--space-md);
	}

	.code-block {
		position: relative;
		background-color: var(--bg-darker);
		border: 1px solid var(--border);
		border-radius: var(--border-radius);
		padding: var(--space-md) var(--space-lg);
		overflow-x: auto;
	}

	.code-block code {
		display: block;
		font-family: var(--font-mono);
		font-size: 0.95rem;
		color: var(--success);
		background: none;
		padding: 0;
		white-space: pre-wrap;
		word-break: break-all;
	}
</style>
