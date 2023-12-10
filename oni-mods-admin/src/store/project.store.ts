import { defineStore } from 'pinia';
import { CreateProjectInfo, SolutionItem } from '../uitls/invokes';

export const useProjectStore = defineStore({
  id: 'project',
  state: (): {
    createProjectInfo: CreateProjectInfo;
    solutionItem: SolutionItem;
  } => ({
    createProjectInfo: {
      root: 'usr/join',
      project_name: '',
      solution_name: '',
    },
    solutionItem: {
      name: '',
      path: '',
    },
  }),
});
