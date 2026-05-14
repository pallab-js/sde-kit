import type { Project, Task, Milestone } from '$lib/types';

async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
	try {
		const { invoke } = await import('@tauri-apps/api/core');
		return await invoke<T>(cmd, args);
	} catch {
		throw new Error('API not available');
	}
}

// Projects
export function getProjects(): Promise<Project[]> { return invoke('get_projects'); }
export function createProject(name: string, path: string, description?: string): Promise<Project> { return invoke('create_project', { name, path, description }); }
export function updateProject(id: string, name?: string, path?: string, description?: string): Promise<void> { return invoke('update_project', { id, name, path, description }); }
export function deleteProject(id: string): Promise<void> { return invoke('delete_project', { id }); }

// Tasks
export function getTasks(): Promise<Task[]> { return invoke('get_tasks'); }
export function getTasksByProject(projectId: string): Promise<Task[]> { return invoke('get_tasks_by_project', { projectId }); }
export function createTask(title: string, description?: string, priority?: string, projectId?: string): Promise<Task> { return invoke('create_task', { title, description, priority, projectId }); }
export function updateTask(id: string, title?: string, description?: string, priority?: string): Promise<void> { return invoke('update_task', { id, title, description, priority }); }
export function updateTaskStatus(id: string, status: string): Promise<void> { return invoke('update_task_status', { id, status }); }
export function deleteTask(id: string): Promise<void> { return invoke('delete_task', { id }); }

// Milestones
export function getMilestones(): Promise<Milestone[]> { return invoke('get_milestones'); }
export function createMilestone(title: string, description?: string, dueDate?: string, projectId?: string): Promise<Milestone> { return invoke('create_milestone', { title, description, dueDate, projectId }); }
export function updateMilestoneStatus(id: string, status: string): Promise<void> { return invoke('update_milestone_status', { id, status }); }
export function deleteMilestone(id: string): Promise<void> { return invoke('delete_milestone', { id }); }
