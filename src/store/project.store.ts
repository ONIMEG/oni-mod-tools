import { defineStore } from 'pinia';
import { CreateProjectInfo } from '../uitls/invokes';

export const useProjectStore = defineStore({
  id: 'project',
  state: (): { createProjectInfo: CreateProjectInfo } => ({
    createProjectInfo: {
      root: 'usr/join',
      project_name: '',
      solution_name: '',
    },
  }),
});
