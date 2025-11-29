<footer>
	<div class="container">
		<div class="footer-content">
			<div class="footer-section">
				<h4>ez-term</h4>
				<p>Privacy-first AI CLI command generator</p>
				<p>Runs 100% locally with Ollama</p>
				<p class="footer-privacy">No tracking. No cookies. No telemetry.</p>
			</div>
			<div class="footer-section">
				<h4>Project</h4>
				<ul>
					<li><a href="https://github.com/ezcorp-org/ez-term" target="_blank" rel="noopener noreferrer">GitHub</a></li>
					<li><a href="https://github.com/ezcorp-org/ez-term#-documentation" target="_blank" rel="noopener noreferrer">Documentation</a></li>
					<li><a href="https://github.com/ezcorp-org/ez-term/blob/main/CONTRIBUTING.md" target="_blank" rel="noopener noreferrer">Contributing</a></li>
					<li><a href="https://github.com/ezcorp-org/ez-term/issues" target="_blank" rel="noopener noreferrer">Issues</a></li>
				</ul>
			</div>
			<div class="footer-section">
				<h4>Resources</h4>
				<ul>
					<li><a href="https://ollama.ai" target="_blank" rel="noopener noreferrer">Ollama</a></li>
					<li><a href="https://github.com/ezcorp-org/ez-term/blob/main/README.md" target="_blank" rel="noopener noreferrer">README</a></li>
					<li><a href="https://github.com/ezcorp-org/ez-term/blob/main/LICENSE" target="_blank" rel="noopener noreferrer">MIT License</a></li>
				</ul>
			</div>
			<div class="footer-section">
				<h4>Community</h4>
				<ul>
					<li><a href="https://discord.ezcorp.org" target="_blank" rel="noopener noreferrer">💬 Discord</a></li>
					<li><a href="https://github.com/ezcorp-org/ez-term/discussions" target="_blank" rel="noopener noreferrer">Discussions</a></li>
				</ul>
			</div>
		</div>
		<div class="footer-bottom">
			<p>Made with ❤️ by <a href="https://ezcorp.org" target="_blank" rel="noopener noreferrer"><strong>EZ Corp</strong></a></p>
		</div>
	</div>
</footer>

<style>
	footer {
		background-color: var(--bg-darker);
		border-top: 1px solid var(--border);
		padding: var(--space-2xl) 0 var(--space-md);
		margin-top: var(--space-3xl);
	}

	.footer-content {
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		gap: var(--space-xl);
		margin-bottom: var(--space-xl);
	}

	.footer-section h4 {
		color: var(--text-primary);
		margin-bottom: var(--space-sm);
		font-size: 1.1rem;
	}

	.footer-section p {
		color: var(--text-secondary);
		margin-bottom: var(--space-xs);
	}

	.footer-privacy {
		margin-top: var(--space-sm);
		color: var(--success);
		font-weight: 500;
	}

	.footer-section ul {
		list-style: none;
	}

	.footer-section li {
		margin-bottom: var(--space-xs);
	}

	.footer-section a {
		color: var(--text-secondary);
		transition: color 0.2s ease;
	}

	.footer-section a:hover {
		color: var(--primary);
	}

	.footer-bottom {
		text-align: center;
		padding-top: var(--space-xl);
		border-top: 1px solid var(--border);
		color: var(--text-secondary);
	}

	.footer-bottom strong {
		color: var(--text-primary);
	}

	.footer-bottom a {
		color: inherit;
		transition: color 0.2s ease;
	}

	.footer-bottom a:hover {
		color: var(--primary);
	}

	.footer-bottom a:hover strong {
		color: var(--primary);
	}

	@media (max-width: 768px) {
		.footer-content {
			grid-template-columns: 1fr;
			gap: var(--space-lg);
		}
	}
</style>
