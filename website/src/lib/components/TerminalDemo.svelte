<script lang="ts">
	import { onMount } from 'svelte';

	const demos = [
		{
			command: 'ez "find large files"',
			description: 'Finds files larger than 100MB in current directory and shows their size in human-readable format.',
			output: 'find . -type f -size +100M -exec ls -lh {} \\;'
		},
		{
			command: 'ez "show git branches"',
			description: 'Lists all git branches with the current branch highlighted.',
			output: 'git branch -a'
		},
		{
			command: 'ez "list running docker containers"',
			description: 'Shows all currently running Docker containers with their status.',
			output: 'docker ps'
		},
		{
			command: 'ez "compress this directory"',
			description: 'Creates a compressed tar.gz archive of the current directory.',
			output: 'tar -czf archive.tar.gz .'
		},
		{
			command: 'ez "check disk usage"',
			description: 'Displays disk usage for the current directory in human-readable format.',
			output: 'du -sh *'
		}
	];

	let currentDemoIndex = 0;
	let terminalLines = $state<string[]>([]);
	let isTyping = $state(false);

	function sleep(ms: number): Promise<void> {
		return new Promise(resolve => setTimeout(resolve, ms));
	}

	async function typeText(text: string): Promise<string> {
		let typed = '';
		for (let char of text) {
			if (!isTyping) break;
			typed += char;
			await sleep(50);
		}
		return typed;
	}

	async function runDemo(demo: typeof demos[0]) {
		isTyping = true;
		terminalLines = [];

		// Type the command
		const commandLine = `$ ${demo.command}`;
		let typed = '$ ';
		for (let char of demo.command) {
			if (!isTyping) break;
			typed += char;
			terminalLines = [typed + '█'];
			await sleep(50);
		}

		terminalLines = [commandLine];
		await sleep(500);

		// Show description
		terminalLines = [...terminalLines, `# ${demo.description}`];
		await sleep(800);

		// Show output
		terminalLines = [...terminalLines, '', `> ${demo.output}`, ''];
		await sleep(3000);

		isTyping = false;
	}

	async function runDemoLoop() {
		while (true) {
			await runDemo(demos[currentDemoIndex]);
			currentDemoIndex = (currentDemoIndex + 1) % demos.length;
			await sleep(1000);
		}
	}

	onMount(() => {
		runDemoLoop();
		return () => {
			isTyping = false;
		};
	});
</script>

<section id="demo" class="demo">
	<div class="container">
		<h2 class="section-title">See it in Action</h2>
		<div class="terminal-window">
			<div class="terminal-header">
				<div class="terminal-buttons">
					<span class="terminal-button terminal-button-close"></span>
					<span class="terminal-button terminal-button-minimize"></span>
					<span class="terminal-button terminal-button-maximize"></span>
				</div>
				<div class="terminal-title">Terminal</div>
			</div>
			<div class="terminal-body">
				<div id="terminal-output">
					{#each terminalLines as line}
						<div class="terminal-line">{line}</div>
					{/each}
				</div>
			</div>
		</div>
	</div>
</section>

<style>
	.demo {
		padding: var(--space-2xl) 0;
	}

	.section-title {
		text-align: center;
		margin-bottom: var(--space-xl);
		color: var(--text-primary);
	}

	.terminal-window {
		max-width: 900px;
		margin: 0 auto;
		background-color: var(--bg-elevated);
		border-radius: var(--border-radius);
		border: 1px solid var(--border);
		overflow: hidden;
		box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
	}

	.terminal-header {
		background-color: var(--bg-darker);
		padding: var(--space-sm) var(--space-md);
		display: flex;
		align-items: center;
		gap: var(--space-md);
		border-bottom: 1px solid var(--border);
	}

	.terminal-buttons {
		display: flex;
		gap: var(--space-xs);
	}

	.terminal-button {
		width: 12px;
		height: 12px;
		border-radius: 50%;
	}

	.terminal-button-close {
		background-color: #FF5F56;
	}

	.terminal-button-minimize {
		background-color: #FFBD2E;
	}

	.terminal-button-maximize {
		background-color: #27C93F;
	}

	.terminal-title {
		color: var(--text-secondary);
		font-size: 0.9rem;
		flex: 1;
		text-align: center;
	}

	.terminal-body {
		padding: var(--space-xl);
		min-height: 300px;
		font-family: var(--font-mono);
		font-size: 0.95rem;
	}

	.terminal-line {
		color: var(--text-primary);
		line-height: 1.6;
		white-space: pre-wrap;
		word-break: break-all;
	}

	.terminal-line:has(.terminal-prompt) {
		color: var(--success);
	}

	@media (max-width: 768px) {
		.terminal-body {
			padding: var(--space-md);
			min-height: 250px;
			font-size: 0.85rem;
		}
	}
</style>
