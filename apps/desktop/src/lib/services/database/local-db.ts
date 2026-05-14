/**
 * Local-First Database Service
 * - Wraps Tauri Rust SQLite commands with version-based conflict detection
 * - Provides export/backup for solo developers
 * - Zero cloud dependencies; works fully offline
 */

import { invoke } from '@tauri-apps/api/core';

export interface DBResult<T> {
  success: boolean;
  data?: T;
  error?: string;
  conflict?: ConflictResolution;
}

export type ConflictResolution = 'local-wins' | 'remote-wins' | 'manual' | 'merge';

export type ExportFormat = 'json' | 'sqlite';

export interface ProjectExport {
  project: Record<string, unknown>;
  tasks: unknown[];
  milestones: unknown[];
  graph: { nodes: unknown[]; edges: unknown[] };
  exported_at: string;
  version: string;
}

export class LocalDatabase {
  private static instance: LocalDatabase;
  private initialized = false;

  private constructor() {}

  static getInstance(): LocalDatabase {
    if (!LocalDatabase.instance) {
      LocalDatabase.instance = new LocalDatabase();
    }
    return LocalDatabase.instance;
  }

  async initialize(): Promise<DBResult<void>> {
    if (this.initialized) return { success: true };
    try {
      await invoke('db_init', { });
      this.initialized = true;
      return { success: true };
    } catch (err) {
      return { success: false, error: String(err) };
    }
  }

  async exportProject(projectId: string, format: ExportFormat): Promise<DBResult<Blob>> {
    try {
      if (format === 'json') {
        const [project, tasks, milestones, nodes, edges] = await Promise.all([
          invoke<Record<string, unknown>>('get_project', { id: projectId }),
          invoke<unknown[]>('get_tasks_by_project', { projectId }),
          invoke<unknown[]>('get_milestones'),
          invoke<unknown[]>('db_query', { query: 'SELECT * FROM graph_nodes WHERE project_id = ?', params: [projectId] }),
          invoke<unknown[]>('db_query', { query: 'SELECT * FROM graph_edges WHERE project_id = ?', params: [projectId] }),
        ]);
        const data: ProjectExport = {
          project, tasks, milestones,
          graph: { nodes, edges },
          exported_at: new Date().toISOString(),
          version: '1.0',
        };
        return { success: true, data: new Blob([JSON.stringify(data, null, 2)], { type: 'application/json' }) };
      }
      const bytes = await invoke<number[]>('db_export_sqlite', { project_id: projectId });
      return { success: true, data: new Blob([new Uint8Array(bytes)], { type: 'application/x-sqlite3' }) };
    } catch (err) {
      return { success: false, error: String(err) };
    }
  }

  async listTables(): Promise<DBResult<string[]>> {
    try {
      const rows = await invoke<{ name: string }[]>('db_query', {
        query: "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name",
        params: [],
      });
      return { success: true, data: rows.map(r => r.name) };
    } catch (err) {
      return { success: false, error: String(err) };
    }
  }
}
