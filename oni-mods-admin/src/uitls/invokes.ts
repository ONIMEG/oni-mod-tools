import { invoke } from '@tauri-apps/api';

export type ResultBody = {
  code: number;
  message: string;
};

export type CreateProjectInfo = {
  root: string;
  project_name: string;
  solution_name: string;
};

export enum StatusCode {
  SUCCESS = 200,
  CONVERT_ERROR = 500,
}

export type SolutionItem = {
  path: string;
  name: string;
};

export type CsprojItem = {
  path: string;
  name: string;
  prop: Project;
};

export type Project = {
  PropertyGroup: Property;
};

type Property = {
  AssemblyTitle: string;
  FileVersion: string;
  RootNamespace: string;
  Description: string;
  AssemblyVersion: string;
  LastWorkingBuild: number;
  Platforms: string;
};

export async function createProject(
  infoJ: CreateProjectInfo,
): Promise<ResultBody> {
  const info: string = JSON.stringify(infoJ);
  const result: string = await invoke<string>('create_project', { info });
  return JSON.parse(result);
}

export async function getSavedSolutionsList(): Promise<ResultBody> {
  const result: string = await invoke<string>('get_solution_list');
  return JSON.parse(result);
}

export async function addNewProject(
  solutionInfoJ: CreateProjectInfo,
  projectInfoJ: Project,
): Promise<ResultBody> {
  const sln: string = JSON.stringify(solutionInfoJ);
  const csproj: string = JSON.stringify(projectInfoJ);
  const result: string = await invoke<string>('add_new_project', {
    sln,
    csproj,
  });
  return JSON.parse(result);
}

export async function getLatestVersion(): Promise<ResultBody> {
  const result: string = await invoke<string>('refresh_version', {});
  return JSON.parse(result);
}

export async function getConfigInfo(): Promise<ResultBody> {
  const result: string = await invoke<string>('get_config_info', {});
  return JSON.parse(result);
}

export async function initGitRepo(info: string): Promise<ResultBody> {
  const result: string = await invoke<string>('git_create_new_repo', {
    info,
  });
  return JSON.parse(result);
}

export async function store_current_project(
  curr_proj: CreateProjectInfo,
): Promise<ResultBody> {
  const currProjInfo: string = JSON.stringify(curr_proj);
  const result: string = await invoke<string>('store_current_proj', {
    currProjInfo,
  });
  return JSON.parse(result);
}

export async function read_current_project_buffer(): Promise<ResultBody> {
  const result: string = await invoke<string>('read_current_proj_buffer', {});
  return JSON.parse(result);
}
