<script setup lang="ts">
import {
  NDropdown,
  NTabs,
  NTabPane,
  NModal,
  NList,
  NListItem,
  NButton,
  NCard,
  useMessage,
} from 'naive-ui';
import { ref } from 'vue';
import CreateCsProject from '../components/CreateCsProject.vue';
import {
  getSavedSolutionsList,
  read_current_project_buffer,
  ResultBody,
  SolutionItem,
  StatusCode,
  store_current_project,
} from '../uitls/invokes';
import { CaretDownFilled as DownArrow, CloseOutlined } from '@vicons/antd';
import { useProjectStore } from '../store/project.store';
import CreateSolution from '../components/CreateSolution.vue';
import GitRepo from '../components/GitRepo.vue';
import { useRouter } from 'vue-router';

const message = useMessage();
const projectInfo = useProjectStore();
const solutionList = ref<SolutionItem[]>([]);
const title = ref<string>('solution-title');
const selectSolutionModalShow = ref<boolean>(false);
const createNewSolutionModalShow = ref<boolean>(false);
const router = useRouter();
const options = ref([
  {
    label: '切换解决方案',
    key: 'open',
  },
  {
    label: '新建解决方案',
    key: 'new',
  },
]);

const select = function (key: string) {
  if (key == 'open') {
    if (solutionList.value.length == 0) getSolutionList();
    selectSolutionModalShow.value = true;
  } else {
    createNewSolutionModalShow.value = true;
  }
};
const selectOneSolution = function (solution: SolutionItem) {
  title.value = solution.name;
  projectInfo.solutionItem = solution;
  projectInfo.createProjectInfo.solution_name = solution.name;
  projectInfo.createProjectInfo.root = solution.path;
  selectSolutionModalShow.value = false;
  const result = store_current_project({
    root: solution.path,
    solution_name: solution.name,
    project_name: '',
  });
  message.info(JSON.stringify(result));
};
const createOneSolution = async function () {
  const result = await store_current_project(projectInfo.createProjectInfo);
  if (result.code != StatusCode.SUCCESS) {
    message.warning('暂存当前项目信息失败' + result.message);
  }
  title.value = projectInfo.createProjectInfo.solution_name;
  createNewSolutionModalShow.value = false;
};

async function getSolutionList() {
  let result: ResultBody = await getSavedSolutionsList();
  if (result.code !== 200) {
    message.error('读取解决方案列表失败！');
    return;
  }
  solutionList.value = JSON.parse(result.message);
}

async function getBufferCurrentProject() {
  let result: ResultBody = await read_current_project_buffer();
  if (result.code != StatusCode.SUCCESS) {
    message.error('读取缓存信息失败！' + result.message);
    if (result.message == '读取失败') {
      await router.push('/first_solution');
    }
    return;
  }
  projectInfo.createProjectInfo = JSON.parse(result.message);
  title.value = projectInfo.createProjectInfo.solution_name;
}

getBufferCurrentProject();
</script>

<template>
  <div>
    <n-modal v-model:show="selectSolutionModalShow">
      <div id="hover-of-select-list">
        <n-list id="solution-list" hoverable clickable bordered>
          <n-list-item
            v-for="solution in solutionList"
            @click="selectOneSolution(solution)"
          >
            {{ solution.name }}
          </n-list-item>
        </n-list>
      </div>
    </n-modal>
    <n-modal v-model:show="createNewSolutionModalShow">
      <n-card title="新建解决方案" size="medium">
        <template #header-extra>
          <n-button
            quaternary
            text
            type="error"
            @click="createNewSolutionModalShow = false"
          >
            <template #icon>
              <CloseOutlined />
            </template>
          </n-button>
        </template>
        <CreateSolution @create="createOneSolution()" />
      </n-card>
    </n-modal>
    <div id="header">
      <n-dropdown
        trigger="hover"
        :options="options"
        :on-select="select"
        size="small"
      >
        <div id="solution-drop-down">
          <span>{{ title }}</span>
          <n-button
            text
            style="color: #737373; margin-top: 2px; font-size: 5px"
          >
            <template #icon><DownArrow /></template>
          </n-button>
        </div>
      </n-dropdown>
    </div>
    <n-tabs :type="'segment'" :placement="'left'">
      <n-tab-pane name="oasis" tab="新建模组" id="create-cs-project">
        <CreateCsProject />
      </n-tab-pane>
      <!--      <n-tab-pane name="the beatles" tab="编译变量"> Hey Jude </n-tab-pane>-->
      <n-tab-pane name="the " tab="仓库">
        <GitRepo />
      </n-tab-pane>
    </n-tabs>
  </div>
</template>

<style scoped>
#header {
  height: 50px;
  width: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
  background: rgb(247, 247, 250);
}

#solution-drop-down {
  cursor: pointer;
  display: flex;
  align-items: center;
}

#solution-drop-down span {
  margin-right: 2px;
}

#create-cs-project {
  padding: 41px 34px 0;
  box-sizing: border-box;
}

#hover-of-select-list {
  height: 60vh;
  background: #ffffff;
  border-radius: 3px;
}

#solution-list {
  width: 60vw;
}

</style>
