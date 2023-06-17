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

export async function getCsprojList(infoJ: SolutionItem): Promise<ResultBody> {
  const info: string = JSON.stringify(infoJ);
  const result: string = await invoke<string>('get_csproj_list', { info });
  return JSON.parse(result);
}
