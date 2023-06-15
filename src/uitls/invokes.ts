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
  CREATE_PROJECT_ERROR = 501,
}

export async function createProject(
  infoJ: CreateProjectInfo,
): Promise<ResultBody> {
  const info: string = JSON.stringify(infoJ);
  const result: string = await invoke<string>('create_project', { info });
  return JSON.parse(result);
}
