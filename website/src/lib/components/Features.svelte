<section id="features" class="features">
	<div class="container">
		<h2 class="section-title">Why Choose ez-term?</h2>
		<div class="features-grid">
			<div class="feature">
				<div class="feature-icon">🔒</div>
				<h3>Privacy First</h3>
				<p>Runs entirely on your machine with Ollama. No data leaves your computer. Zero telemetry. No cloud dependencies.</p>
			</div>
			<div class="feature">
				<div class="feature-icon">🧠</div>
				<h3>Context Aware</h3>
				<p>Understands your environment, tools, and projects automatically. Generates commands tailored to your actual setup.</p>
			</div>
			<div class="feature">
				<div class="feature-icon">🛡️</div>
				<h3>Safe by Default</h3>
				<p>Preview before execute. Defaults to safe, read-only operations. Refuses dangerous commands like <code>rm -rf /</code>.</p>
			</div>
		</div>
	</div>
</section>

<style>
	.features {
		padding: var(--space-2xl) 0;
		background-color: var(--bg-darker);
	}

	.section-title {
		text-align: center;
		margin-bottom: var(--space-xl);
		color: var(--text-primary);
	}

	.features-grid {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
		gap: var(--space-xl);
	}

	.feature {
		background-color: var(--bg-elevated);
		padding: var(--space-xl);
		border-radius: var(--border-radius);
		border: 1px solid var(--border);
		transition: transform 0.2s ease, border-color 0.2s ease;
	}

	.feature:hover {
		transform: translateY(-4px);
		border-color: var(--primary);
	}

	.feature-icon {
		font-size: 3rem;
		margin-bottom: var(--space-md);
	}

	.feature h3 {
		color: var(--text-primary);
		margin-bottom: var(--space-sm);
	}

	.feature p {
		color: var(--text-secondary);
		line-height: 1.6;
	}

	@media (max-width: 768px) {
		.features-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
