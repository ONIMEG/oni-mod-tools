<script setup lang="ts">
import { NList, NListItem, NButton, NIcon, useMessage } from 'naive-ui';
import { dialog } from '@tauri-apps/api';
import {
  AppstoreAddOutlined as NewMod,
  FolderOpenOutlined as OpenMod,
} from '@vicons/antd';
import { useProjectStore } from '../store/project.store';
import { useRouter } from 'vue-router';

const message = useMessage();
const projectInfo = useProjectStore();
const router = useRouter();

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

function open() {
  console.log('sa');
}
</script>

<template>
  <div class="full-height flex flex-all-center">
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
  </div>
</template>

<style scoped>
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
