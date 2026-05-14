<script lang="ts">
	import { onMount } from 'svelte';
	import { listDirectory } from '$lib/services/fs';
	import { openTab } from '$lib/stores/workspace';
	import type { FileEntry } from '$lib/types';
	import FileTree from './FileTree.svelte';

	let { root, pageSize = 50 }: { root: string; pageSize?: number } = $props();

	let children = $state<FileEntry[]>([]);
	let loading = $state(true);
	let expanded = $state(false);
	let showCount = $state(50);

	onMount(() => load());

	async function load() {
		loading = true;
		try {
			children = await listDirectory(root);
		} catch {
			children = [];
		}
		loading = false;
	}

	function toggle() {
		expanded = !expanded;
		if (expanded && children.length === 0) load();
	}

	function openFile(entry: FileEntry) {
		openTab({
			id: `file:${entry.path}`,
			title: entry.name,
			icon: '📄',
			closable: true,
			filePath: entry.path,
		});
	}
</script>

<div class="folder-row" class:expanded>
	<button class="folder-toggle typo-caption" onclick={toggle} aria-label={expanded ? 'Collapse' : 'Expand'}>
		<span class="chevron">{expanded ? '▼' : '▶'}</span>
		<span class="folder-icon">{expanded ? '📂' : '📁'}</span>
		<span class="folder-name">{root.split('/').pop() || root}</span>
	</button>
</div>

{#if expanded}
	<div class="folder-children">
		{#if loading}
			<div class="loading typo-small">Loading...</div>
		{:else}
			{#each children.filter(c => c.is_dir) as dir (dir.path)}
				<div class="child">
					<FileTree root={dir.path} />
				</div>
			{/each}
			{@const files = children.filter(c => !c.is_dir)}
			{#each files.slice(0, showCount) as file (file.path)}
				<button class="file-row typo-caption" onclick={() => openFile(file)}>
					<span class="file-icon">📄</span>
					<span class="file-name">{file.name}</span>
				</button>
			{/each}
			{#if files.length > showCount}
				<button class="show-more typo-small" onclick={() => (showCount += pageSize)}>
					Show {files.length - showCount} more…
				</button>
			{/if}
		{/if}
	</div>
{/if}

<style>
	.folder-row {
		user-select: none;
	}

	.folder-toggle {
		display: flex;
		align-items: center;
		gap: 4px;
		width: 100%;
		padding: 3px var(--spacing-3);
		border: none;
		background: none;
		color: var(--color-on-dark-soft);
		cursor: pointer;
		text-align: left;
	}

	.folder-toggle:hover {
		background: var(--color-surface-dark);
		color: var(--color-on-dark);
	}

	.chevron {
		font-size: 8px;
		width: 12px;
		flex-shrink: 0;
		opacity: 0.6;
	}

	.folder-icon {
		width: 16px;
		text-align: center;
		flex-shrink: 0;
	}

	.folder-name {
		font-weight: 500;
		color: var(--color-on-dark-soft);
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.folder-children {
		padding-left: var(--spacing-3);
	}

	.child {
		margin: 0;
	}

	.file-row {
		display: flex;
		align-items: center;
		gap: 4px;
		width: 100%;
		padding: 3px var(--spacing-3);
		padding-left: 28px;
		border: none;
		background: none;
		color: var(--color-on-dark-soft);
		cursor: pointer;
		text-align: left;
	}

	.file-row:hover {
		background: var(--color-surface-dark);
		color: var(--color-on-dark);
	}

	.file-icon {
		width: 16px;
		text-align: center;
		flex-shrink: 0;
	}

	.file-name {
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.loading {
		padding: 4px var(--spacing-3);
		color: var(--color-muted-soft);
		font-style: italic;
	}

	.show-more {
		display: block;
		width: 100%;
		padding: 3px var(--spacing-3);
		padding-left: 28px;
		border: none;
		background: none;
		color: var(--color-muted);
		cursor: pointer;
		font-style: italic;
		text-align: left;
	}

	.show-more:hover {
		color: var(--color-on-dark);
		background: var(--color-surface-dark);
	}
</style>
