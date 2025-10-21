<script lang="ts">
	import { copyToClipboard } from '$lib/utils/copyToClipboard';

	interface Props {
		text: string;
		inline?: boolean;
	}

	let { text, inline = false }: Props = $props();

	let copied = $state(false);

	async function handleCopy() {
		const success = await copyToClipboard(text);
		if (success) {
			copied = true;
			setTimeout(() => {
				copied = false;
			}, 2000);
		}
	}
</script>

<button
	type="button"
	class="copy-btn"
	class:copy-btn-inline={inline}
	class:copied
	onclick={handleCopy}
	aria-label="Copy to clipboard"
>
	{copied ? '✓ Copied!' : 'Copy'}
</button>

<style>
	.copy-btn {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px 16px;
		background-color: var(--bg-elevated);
		color: var(--text-primary);
		border: 1px solid var(--border);
		border-radius: var(--border-radius);
		font-family: var(--font-sans);
		font-size: 0.9rem;
		cursor: pointer;
		transition: all 0.2s ease;
		min-height: 36px;
	}

	.copy-btn:hover {
		background-color: var(--primary);
		color: var(--bg-dark);
		border-color: var(--primary);
	}

	.copy-btn.copied {
		background-color: var(--success);
		color: var(--bg-dark);
		border-color: var(--success);
	}

	.copy-btn-inline {
		position: absolute;
		top: var(--space-sm);
		right: var(--space-sm);
	}
</style>
