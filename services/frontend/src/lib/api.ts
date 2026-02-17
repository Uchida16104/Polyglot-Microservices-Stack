const API_BASE = import.meta.env.VITE_API_URL ?? 'http://localhost:8000/api';

interface FetchOptions extends RequestInit {
  json?: unknown;
}

export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
    this.name = 'ApiError';
  }
}

async function request<T>(path: string, options: FetchOptions = {}): Promise<T> {
  const token = getToken();

  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    ...(options.headers as Record<string, string>),
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  const body = options.json !== undefined ? JSON.stringify(options.json) : options.body;

  const res = await fetch(`${API_BASE}${path}`, {
    ...options,
    headers,
    body,
  });

  if (res.status === 401) {
    clearAuth();
    window.location.href = '/login';
    throw new ApiError(401, 'Unauthorized');
  }

  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: res.statusText }));
    throw new ApiError(res.status, err.message ?? 'Request failed');
  }

  if (res.status === 204) return null as T;
  return res.json();
}

function getToken(): string | null {
  if (typeof document === 'undefined') return null;
  return document.cookie
    .split('; ')
    .find(row => row.startsWith('access_token='))
    ?.split('=')[1] ?? null;
}

function setToken(token: string): void {
  document.cookie = `access_token=${token}; path=/; max-age=86400; SameSite=Strict`;
}

function clearAuth(): void {
  document.cookie = 'access_token=; path=/; max-age=0';
  document.cookie = 'refresh_token=; path=/; max-age=0';
}

export const auth = {
  login: (email: string, password: string) =>
    request<{ user: User; access_token: string; refresh_token: string }>(
      '/auth/login', { method: 'POST', json: { email, password } }
    ).then(data => { setToken(data.access_token); return data; }),

  register: (name: string, email: string, password: string, password_confirmation: string) =>
    request<{ user: User; access_token: string }>('/auth/register', {
      method: 'POST', json: { name, email, password, password_confirmation },
    }).then(data => { setToken(data.access_token); return data; }),

  logout: async () => {
    await request('/auth/logout', { method: 'POST' }).catch(() => {});
    clearAuth();
  },

  me: () => request<{ user: User }>('/auth/me'),
};

export const projects = {
  list: (page = 1) => request<Project[]>(`/projects?page=${page}`),
  get: (id: string) => request<Project>(`/projects/${id}`),
  create: (data: CreateProjectData) => request<Project>('/projects', { method: 'POST', json: data }),
  update: (id: string, data: Partial<CreateProjectData>) =>
    request<Project>(`/projects/${id}`, { method: 'PUT', json: data }),
  delete: (id: string) => request<null>(`/projects/${id}`, { method: 'DELETE' }),
  members: (id: string) => request<ProjectMember[]>(`/projects/${id}/members`),
};

export const tasks = {
  list: (projectId: string, status?: string) =>
    request<Task[]>(`/projects/${projectId}/tasks${status ? `?status=${status}` : ''}`),
  create: (projectId: string, data: CreateTaskData) =>
    request<Task>(`/projects/${projectId}/tasks`, { method: 'POST', json: data }),
  update: (projectId: string, taskId: string, data: Partial<Task>) =>
    request<Task>(`/projects/${projectId}/tasks/${taskId}`, { method: 'PUT', json: data }),
  delete: (projectId: string, taskId: string) =>
    request<null>(`/projects/${projectId}/tasks/${taskId}`, { method: 'DELETE' }),
};

export const compile = {
  submit: (data: CompileJobData) =>
    request<{ job_id: string; status: string }>('/compile', { method: 'POST', json: data }),
  status: (jobId: string) => request<CompileJob>(`/compile/${jobId}/status`),
  execute: (jobId: string, stdinData?: string) =>
    request<ExecuteResult>('/compile/execute', { method: 'POST', json: { compile_job_id: jobId, stdin_data: stdinData } }),
};

export const ml = {
  listModels: () => request<MlModel[]>('/ml/models'),
  createModel: (data: CreateModelData) => request<MlModel>('/ml/models', { method: 'POST', json: data }),
  listJobs: () => request<MlJob[]>('/ml/jobs'),
  createJob: (data: CreateJobData) => request<MlJob>('/ml/jobs', { method: 'POST', json: data }),
  getJob: (id: string) => request<MlJob>(`/ml/jobs/${id}`),
};

export interface User { id: string; name: string; email: string; role: string; }
export interface Project { id: string; owner_id: string; name: string; description?: string; visibility: string; status: string; created_at: string; }
export interface ProjectMember { id: string; project_id: string; user_id: string; role: string; }
export interface Task { id: string; project_id: string; title: string; description?: string; priority: string; status: string; assignee_id?: string; due_at?: string; order_index: number; }
export interface CompileJob { id: string; status: string; language: string; output?: string; error_output?: string; exit_code?: number; duration_ms?: number; }
export interface ExecuteResult { job_id: string; status: string; stdout: string; stderr: string; exit_code: number; duration_ms: number; }
export interface MlModel { id: string; name: string; framework: string; version: string; status: string; }
export interface MlJob { id: string; model_id: string; job_type: string; status: string; result_metrics?: Record<string, unknown>; duration_ms?: number; }
export interface CreateProjectData { name: string; description?: string; visibility?: string; }
export interface CreateTaskData { title: string; description?: string; priority?: string; due_at?: string; }
export interface CompileJobData { project_id: string; language: string; source_code: string; compiler_flags?: string; }
export interface CreateModelData { name: string; framework: string; version?: string; hyperparams?: Record<string, unknown>; }
export interface CreateJobData { model_id: string; job_type: string; config?: Record<string, unknown>; }
