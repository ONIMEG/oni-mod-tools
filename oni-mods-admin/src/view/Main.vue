<script setup lang="ts">
import { NList, NListItem, NButton, NIcon, useMessage } from 'naive-ui';
import { dialog } from '@tauri-apps/api';
import {
  AppstoreAddOutlined as NewMod,
  FolderOpenOutlined as OpenMod,
} from '@vicons/antd';
import { useProjectStore } from '../store/project.store';
import { useRouter } from 'vue-router';
import {
  getSavedSolutionsList,
  ResultBody,
  SolutionItem,
} from '../uitls/invokes';
import { ref } from 'vue';

const message = useMessage();
const projectInfo = useProjectStore();
const router = useRouter();
const showList = ref<boolean>(false);
const solutionList = ref<SolutionItem[]>([]);

async function openSelectedSolution(item: SolutionItem) {
  projectInfo.solutionItem = item;
  projectInfo.createProjectInfo.solution_name = item.name;
  projectInfo.createProjectInfo.root = item.path;
  await router.push('/solution');
}

async function getSolutionList() {
  let result: ResultBody = await getSavedSolutionsList();
  if (result.code !== 200) {
    message.error('读取解决方案列表失败！');
    return;
  }
  solutionList.value = JSON.parse(result.message);
  if (solutionList.value.length !== 0) {
    showList.value = true;
  }
}

async function create() {
  const path = await dialog.open({
    title: '选择创建位置',
    multiple: false,
    directory: true,
  });
  if (path === null) return;
  if (Array.isArray(path)) {
    message.warning('请选择单个目录');
    return;
  }
  projectInfo.createProjectInfo.root = path;
  await router.push('/create_solution');
}

async function open() {
  const path = await dialog.open({
    title: '选择要打开的目录',
    multiple: false,
    directory: true,
  });
  if (path === null) return;
  if (Array.isArray(path)) {
    message.warning('请选择单个目录');
    return;
  }
}

getSolutionList();
</script>

<template>
  <div class="full-height flex flex-all-center" v-if="!showList">
    <div id="create" class="center-offset">
      <h2 class="title">开始</h2>
      <div id="create-items">
        <div>
          <n-button
            strong
            secondary
            type="primary"
            class="c-item"
            @click="create"
          >
            <n-icon>
              <NewMod />
            </n-icon>
          </n-button>
          <div class="c-item-hint">新建</div>
        </div>
        <div>
          <n-button
            strong
            secondary
            type="primary"
            class="c-item"
            @click="open"
          >
            <n-icon>
              <OpenMod />
            </n-icon>
          </n-button>
          <div class="c-item-hint">打开</div>
        </div>
      </div>
    </div>
    <div id="recent" class="flex-1">
      <h2 class="title">最近使用</h2>
      <n-list hoverable clickable>
        <n-list-item class="recent-list-item">
          <h2>title</h2>
          <span>location</span>
        </n-list-item>
      </n-list>
    </div>
    <div class="setting-button">
      <n-button
        @click="
        () => {
          router.push('/setting');
        }
      "
      >设置</n-button
      >
    </div>
  </div>
  <div v-else>
    <div id="create-nav">
      <n-button strong secondary type="primary" @click="create">
        新建
      </n-button>
    </div>
    <n-list hoverable clickable>
      <template v-for="item in solutionList">
        <n-list-item
          class="solution-list-item"
          @click="openSelectedSolution(item)"
        >
          <div class="flex">
            <span>{{ item.name[0] }}</span>
            <div class="base-info">
              <div class="title">{{ item.name }}</div>
              <div class="path">{{ item.path }}</div>
            </div>
          </div>
        </n-list-item>
      </template>
    </n-list>
  </div>
</template>

<style scoped>

.solution-list-item {
  padding: 10px 1rem;
  cursor: pointer;
}

.solution-list-item .base-info {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  margin-left: 10px;
}

.solution-list-item .base-info .title {
  font-weight: 600;
  font-size: 1.2rem;
}

.solution-list-item .base-info .path {
  font-size: 0.6rem;
  color: #414141;
}

.solution-list-item span {
  --length: 3rem;
  display: inline-block;
  height: var(--length);
  width: var(--length);
  text-align: center;
  line-height: var(--length);
  border-radius: 5px;
  background: #efefef;
  font-size: 1.2rem;
  font-weight: 600;
}

#create-nav {
  display: flex;
  gap: 10px;
  padding: 10px 1rem;
  border-bottom: 1px solid #efefef;
}

.title {
  font-weight: 400;
}

#create {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.c-item {
  --height-c-item: 4rem;
  height: var(--height-c-item);
  width: var(--height-c-item);
  border-radius: 5px;
  font-size: 1.5rem;
  font-weight: bold;
}

.c-item-hint {
  text-align: center;
  margin-top: 10px;
  font-size: 0.8rem;
}

#create-items {
  display: flex;
  gap: 3rem;
}

#recent {
  display: none;
}

#recent .title {
  font-size: 0.6rem;
  background-color: #efefef;
  color: #2f2f2f;
  padding: 2px 10px;
}

#recent .recent-list-item h2 {
  margin: 0;
}

#recent .recent-list-item span {
  font-size: 0.4rem;
  color: #8b8b8b;
}
</style>
