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
				<a href="https://github.com/ezcorp-org/ez-term" target="_blank" rel="noopener noreferrer" class="github-link">
					<svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor">
						<path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
					</svg>
					<span>Star on GitHub</span>
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

	.github-link {
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

	.github-link:hover {
		background-color: var(--primary);
		color: var(--bg-dark);
		border-color: var(--primary);
		transform: translateY(-2px);
	}

	@media (max-width: 768px) {
		.logo-text {
			font-size: 1.2rem;
		}

		.github-link span {
			display: none;
		}
	}
</style>
