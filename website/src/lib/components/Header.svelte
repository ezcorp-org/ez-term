<script lang="ts">
	import { base } from '$app/paths';
</script>

<header>
	<div class="container">
		<div class="header-content">
			<div class="logo">
				<img src="{base}/assets/images/ez-term-logo.svg" alt="ez-term" class="logo-img" />
			</div>
			<nav>
				<a href="https://discord.ezcorp.org" target="_blank" rel="noopener noreferrer" class="nav-link discord-link">
					<svg width="24" height="24" viewBox="0 0 24 24" fill="currentColor">
						<path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515a.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0a12.64 12.64 0 0 0-.617-1.25a.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057a19.9 19.9 0 0 0 5.993 3.03a.078.078 0 0 0 .084-.028a14.09 14.09 0 0 0 1.226-1.994a.076.076 0 0 0-.041-.106a13.107 13.107 0 0 1-1.872-.892a.077.077 0 0 1-.008-.128a10.2 10.2 0 0 0 .372-.292a.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127a12.299 12.299 0 0 1-1.873.892a.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028a19.839 19.839 0 0 0 6.002-3.03a.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03zM8.02 15.33c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.956-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.956 2.418-2.157 2.418zm7.975 0c-1.183 0-2.157-1.085-2.157-2.419c0-1.333.955-2.419 2.157-2.419c1.21 0 2.176 1.096 2.157 2.42c0 1.333-.946 2.418-2.157 2.418z"/>
					</svg>
					<span>Discord</span>
				</a>
				<a href="https://github.com/ezcorp-org/ez-term" target="_blank" rel="noopener noreferrer" class="nav-link github-link">
					<svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor">
						<path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
					</svg>
					<span>GitHub</span>
				</a>
			</nav>
		</div>
	</div>
</header>

<style>
	header {
		background-color: var(--bg-darker);
		border-bottom: 1px solid var(--border);
		padding: var(--space-md) 0;
		position: sticky;
		top: 0;
		z-index: 100;
		backdrop-filter: blur(8px);
		background-color: rgba(1, 4, 9, 0.95);
	}

	.header-content {
		display: flex;
		justify-content: space-between;
		align-items: center;
	}

	.logo {
		display: flex;
		align-items: center;
		gap: var(--space-sm);
	}

	.logo-img {
		height: 40px;
		width: auto;
	}

	.logo-text {
		font-size: 1.5rem;
		font-weight: 600;
		color: var(--text-primary);
		font-family: var(--font-mono);
	}

	nav {
		display: flex;
		gap: var(--space-sm);
	}

	.nav-link {
		display: inline-flex;
		align-items: center;
		gap: var(--space-xs);
		padding: 10px 20px;
		background-color: var(--bg-elevated);
		border: 1px solid var(--border);
		border-radius: var(--border-radius);
		color: var(--text-primary);
		font-weight: 500;
		transition: all 0.2s ease;
		min-height: 44px;
	}

	.nav-link:hover {
		background-color: var(--primary);
		color: var(--bg-dark);
		border-color: var(--primary);
		transform: translateY(-2px);
	}

	.discord-link:hover {
		background-color: #5865F2;
		border-color: #5865F2;
	}

	@media (max-width: 768px) {
		.logo-text {
			font-size: 1.2rem;
		}

		nav {
			gap: var(--space-xs);
		}

		.nav-link {
			padding: 10px 12px;
		}

		.nav-link span {
			display: none;
		}
	}
</style>
