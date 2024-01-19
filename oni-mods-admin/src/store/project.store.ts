import { defineStore } from 'pinia';
import { CreateProjectInfo, CsprojItem, SolutionItem } from '../uitls/invokes';

export const useProjectStore = defineStore({
  id: 'project',
  state: (): {
    createProjectInfo: CreateProjectInfo;
    solutionItem: SolutionItem;
    currentCsProjInfo: CsprojItem;
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
    currentCsProjInfo: {
      name: '',
      path: '',
      prop: {
        PropertyGroup: {
          AssemblyTitle: '',
          FileVersion: '1.0.0',
          RootNamespace: '',
          Description: '',
          AssemblyVersion: '1.0.0',
          LastWorkingBuild: 526233,
          Platforms: 'Vanilla;Mergedown',
        },
      },
    },
  }),
});
