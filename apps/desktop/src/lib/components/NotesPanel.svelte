<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';

	const NOTE_ID = 'scratch';
	let content = $state('');
	let status = $state<'saved' | 'saving' | 'unsaved'>('saved');
	let saveTimer: ReturnType<typeof setTimeout> | undefined;

	async function load() {
		try {
			const val = await invoke<string | null>('get_note', { noteId: NOTE_ID });
			content = val ?? '';
		} catch {}
	}

	async function save() {
		status = 'saving';
		try {
			await invoke('save_note', { noteId: NOTE_ID, content });
			status = 'saved';
		} catch { status = 'unsaved'; }
	}

	function onInput() {
		status = 'unsaved';
		clearTimeout(saveTimer);
		saveTimer = setTimeout(save, 800);
	}

	load();
</script>

<div class="notes-panel">
	<div class="notes-header">
		<span class="notes-title typo-overline">SCRATCH PAD</span>
		<span class="notes-status typo-small">{status === 'saving' ? '⟳ Saving' : status === 'saved' ? '✓' : '●'}</span>
	</div>
	<textarea
		class="notes-body typo-body"
		placeholder="Write notes, ideas, snippets..."
		bind:value={content}
		oninput={onInput}
	></textarea>
</div>

<style>
	.notes-panel { display: flex; flex-direction: column; height: 100%; }
	.notes-header { display: flex; justify-content: space-between; align-items: center; padding: 6px var(--spacing-3); border-bottom: 1px solid var(--color-surface-dark-border); }
	.notes-title { color: var(--color-muted); }
	.notes-status { color: var(--color-muted); }
	.notes-body { flex: 1; background: var(--color-surface-dark); color: var(--color-on-dark); border: none; outline: none; resize: none; padding: var(--spacing-3); font-family: var(--font-mono); font-size: 13px; line-height: 1.6; }
</style>
