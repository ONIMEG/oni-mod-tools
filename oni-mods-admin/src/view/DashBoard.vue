<script setup lang="ts">
import {
  NDropdown,
  NTabs,
  NTabPane,
  NModal,
  NList,
  NListItem,
  NButton,
  useMessage,
} from 'naive-ui';
import { ref } from 'vue';
import CreateCsProject from '../components/CreateCsProject.vue';
import {
  getSavedSolutionsList,
  ResultBody,
  SolutionItem,
} from '../uitls/invokes';
import { CaretDownFilled as DownArrow } from '@vicons/antd';
import { useProjectStore } from '../store/project.store';

const message = useMessage();
const projectInfo = useProjectStore();
const solutionList = ref<SolutionItem[]>([]);
const title = ref<string>('solution-title');
const selectSolutionModalShow = ref<boolean>(true);
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
  }
};
const selectOneSolution = function (solution: SolutionItem) {
  title.value = solution.name;
  projectInfo.solutionItem = solution;
  projectInfo.createProjectInfo.solution_name = solution.name;
  projectInfo.createProjectInfo.root = solution.path;
  selectSolutionModalShow.value = false;
};

async function getSolutionList() {
  let result: ResultBody = await getSavedSolutionsList();
  if (result.code !== 200) {
    message.error('读取解决方案列表失败！');
    return;
  }
  solutionList.value = JSON.parse(result.message);
}
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
      <n-tab-pane name="the beatles" tab="编译变量"> Hey Jude </n-tab-pane>
      <n-tab-pane name="the " tab="仓库"> Hey Judett </n-tab-pane>
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

#solution-drop-down span{
  margin-right: 2px;
}

#create-cs-project {
  padding: 16px 12px 0;
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
