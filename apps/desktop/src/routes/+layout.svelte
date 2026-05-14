<script lang="ts">
	import { onMount } from 'svelte';
	import { theme } from '$lib/stores/theme';
	import { undoManager } from '$lib/services/undoManager';
	import ErrorBoundary from '$lib/components/ErrorBoundary.svelte';
	import '../app.css';

	let { children } = $props();

	onMount(() => {
		theme.init();

		function handleKey(e: KeyboardEvent) {
			const meta = e.metaKey || e.ctrlKey;
			if (meta && e.shiftKey && e.key === 'z') {
				e.preventDefault();
				undoManager.redo();
			} else if (meta && e.key === 'z') {
				e.preventDefault();
				undoManager.undo();
			}
		}

		document.addEventListener('keydown', handleKey);
		return () => document.removeEventListener('keydown', handleKey);
	});
</script>

<ErrorBoundary>
	<div class="app-shell">
		{@render children()}
	</div>
</ErrorBoundary>

<style>
	.app-shell {
		display: contents;
	}
</style>
