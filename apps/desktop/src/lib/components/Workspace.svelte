<script lang="ts">
	import { onMount } from 'svelte';
	import CommandBar from './CommandBar.svelte';
	import ActivityBar from './ActivityBar.svelte';
	import Sidebar from './Sidebar.svelte';
	import MainContent from './MainContent.svelte';
	import BottomPanel from './BottomPanel.svelte';
	import CommandPalette from './CommandPalette.svelte';
	import KeyboardShortcuts from './KeyboardShortcuts.svelte';
	import { activePanel, sidebarWidth, bottomPanelOpen } from '$lib/stores/workspace';
	import { restoreLayout, subscribeAndPersist } from '$lib/stores/layout';

	let paletteOpen = $state(false);

	onMount(() => {
		sidebarWidth.set(window.innerWidth / 2);
		bottomPanelOpen.set(false);
		restoreLayout();
		subscribeAndPersist();
	});
</script>

<KeyboardShortcuts onCommandPalette={() => (paletteOpen = true)} />
<CommandPalette open={paletteOpen} onClose={() => (paletteOpen = false)} />

<div class="workspace">
	<CommandBar onPalette={() => (paletteOpen = true)} />
	<div class="workspace-body">
		<ActivityBar />
		{#if $activePanel}
			<Sidebar />
		{/if}
		<MainContent />
	</div>
	<BottomPanel />
</div>

<style>
	.workspace {
		display: flex;
		flex-direction: column;
		height: 100vh;
		width: 100vw;
		overflow: hidden;
	}

	.workspace-body {
		display: flex;
		flex: 1;
		overflow: hidden;
	}
</style>
