import { writable } from 'svelte/store';

export interface EditorState {
	path: string;
	content: string;
	dirty: boolean;
}

export const fileContents = writable<Map<string, EditorState>>(new Map());
export const splitMode = writable<boolean>(false);
export const splitRatio = writable<number>(0.5);

export function setFileContent(path: string, content: string) {
	fileContents.update((map) => {
		const existing = map.get(path);
		map.set(path, {
			path,
			content,
			dirty: existing?.dirty ?? false,
		});
		return new Map(map);
	});
}

export function markDirty(path: string, dirty: boolean) {
	fileContents.update((map) => {
		const entry = map.get(path);
		if (entry) {
			entry.dirty = dirty;
			return new Map(map);
		}
		return map;
	});
}

export function updateContent(path: string, content: string) {
	fileContents.update((map) => {
		const existing = map.get(path);
		if (existing) {
			map.set(path, { ...existing, content, dirty: true });
			return new Map(map);
		}
		return map;
	});
}

export function getFileState(path: string): EditorState | undefined {
	let result: EditorState | undefined;
	fileContents.subscribe((m) => (result = m.get(path)))();
	return result;
}
