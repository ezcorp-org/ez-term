<section id="how-it-works" class="how-it-works">
	<div class="container">
		<h2 class="section-title">How It Works</h2>
		<div class="steps">
			<div class="step">
				<div class="step-number">1</div>
				<h3>Install</h3>
				<p>One-line installation. No Rust required.</p>
				<div class="code-block">
					<code>curl -sSL https://raw.githubusercontent.com/ezcorp-org/ez-term/main/scripts/install.sh | bash</code>
				</div>
			</div>
			<div class="step">
				<div class="step-number">2</div>
				<h3>Configure Ollama</h3>
				<p>Set up local AI in minutes.</p>
				<div class="code-block">
					<code>curl -fsSL https://ollama.ai/install.sh | sh</code>
					<code>ollama pull qwen2.5-coder:latest</code>
					<code>ez --set-backend ollama</code>
				</div>
			</div>
			<div class="step">
				<div class="step-number">3</div>
				<h3>Use</h3>
				<p>Ask in natural language. Get perfect commands.</p>
				<div class="code-block">
					<code>ez "find large files"</code>
					<code>ez "show git branches"</code>
					<code>ez "compress this directory"</code>
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.how-it-works {
		padding: var(--space-2xl) 0;
	}

	.section-title {
		text-align: center;
		margin-bottom: var(--space-xl);
		color: var(--text-primary);
	}

	.steps {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
		gap: var(--space-xl);
	}

	.step {
		background-color: var(--bg-elevated);
		padding: var(--space-xl);
		border-radius: var(--border-radius);
		border: 1px solid var(--border);
		position: relative;
	}

	.step-number {
		position: absolute;
		top: -20px;
		left: var(--space-xl);
		width: 40px;
		height: 40px;
		background: linear-gradient(135deg, var(--primary), var(--success));
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		font-weight: 700;
		font-size: 1.2rem;
		color: var(--bg-dark);
	}

	.step h3 {
		color: var(--text-primary);
		margin: var(--space-md) 0;
	}

	.step p {
		color: var(--text-secondary);
		margin-bottom: var(--space-md);
	}

	.code-block {
		background-color: var(--bg-darker);
		border: 1px solid var(--border);
		border-radius: var(--border-radius);
		padding: var(--space-md);
		overflow-x: auto;
	}

	.code-block code {
		display: block;
		font-family: var(--font-mono);
		font-size: 0.9rem;
		color: var(--success);
		background: none;
		padding: 4px 0;
		white-space: pre-wrap;
		word-break: break-all;
	}

	.code-block code + code {
		margin-top: var(--space-xs);
	}

	@media (max-width: 768px) {
		.steps {
			grid-template-columns: 1fr;
		}
	}
</style>
