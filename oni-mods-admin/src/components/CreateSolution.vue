<script setup lang="ts">
import {
  NForm,
  NFormItem,
  NInput,
  NButton,
  NRadio,
  NInputGroup,
  FormInst,
  useMessage,
} from 'naive-ui';
import { computed, ref } from 'vue';
import {
  CreateProjectInfo,
  createProject,
  ResultBody,
  StatusCode,
  initGitRepo,
} from '../uitls/invokes';
import { useRouter } from 'vue-router';
import { dialog } from '@tauri-apps/api';
import { useProjectStore } from '../store/project.store';

const router = useRouter();
const formRef = ref<FormInst | null>(null);
const formInfo = ref<CreateProjectInfo>({
  root: '',
  project_name: '',
  solution_name: 'ONI-Mods',
});
const projectInfo = useProjectStore();
const loadingVisible = ref<boolean>(false);
const message = useMessage();
const createGitRepo = ref<boolean>(false);
const reg: RegExp = /^[a-zA-Z0-9_-]+$/;
const pathReg: RegExp = /^[a-zA-Z]:\\((?:[^\\"<>|?*:]+\\)+)$/;
const emit = defineEmits(['create']);

function createStatus(value: string) {
  if (!value || !reg.test(value)) {
    return 'error';
  }
  return 'success';
}

function createPathStatus(value: string) {
  if (!value || !reg.test(value)) {
    return 'error';
  }
  return 'success';
}

function createFeedback(value: string) {
  if (!value) {
    return '需要填写';
  }
  if (!reg.test(value)) {
    return '只能包含数字、字母、下划线、减号';
  }
}

async function create(e: MouseEvent) {
  e.preventDefault();
  if (formInfo.value.root == '')
    if (
      createStatus(formInfo.value.solution_name) === 'error' ||
      createPathStatus(formInfo.value.root) === 'error'
    ) {
      message.warning('请正确填写信息');
      return;
    }
  loadingVisible.value = true;
  const result: ResultBody = await createProject(formInfo.value);
  loadingVisible.value = false;
  if (result.code === StatusCode.SUCCESS) {
    message.success('成功');
    projectInfo.createProjectInfo = formInfo.value;
    emit('create', true);
    if (createGitRepo.value) {
      console.log(formInfo.value);
      await gitRepoInit(
        formInfo.value.root + '//' + formInfo.value.solution_name,
      );
      router.back();
    }
    return;
  }
  if (result.code === StatusCode.CONVERT_ERROR) {
    message.error('格式转换失败：' + result.message);
  } else {
    message.error(`项目创建失败：${result.message}`);
  }
}

async function gitRepoInit(repoPath: string) {
  console.log(repoPath);
  const result = await initGitRepo(repoPath);
  if (result.code == StatusCode.SUCCESS) {
    message.success('初始化 Git 仓库成功');
  } else {
    message.warning('项目创建成功但初始化 Git 仓库失败');
  }
}

async function selectPath() {
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
  formInfo.value.root = path;
}
</script>

<template>
  <n-form
    label-placement="left"
    label-width="auto"
    ref="formRef"
    style="width: 60vw"
  >
    <n-form-item label="路径">
      <n-input-group>
        <n-input
          v-model:value="formInfo.root"
          placeholder="新建模组项目集位置"
        />
        <n-button tertiary @click="selectPath"> 选择 </n-button>
      </n-input-group>
    </n-form-item>
    <n-form-item
      label="名称"
      :validation-status="
        computed(() => {
          return createStatus(formInfo.solution_name);
        }).value
      "
      :feedback="
        computed(() => {
          return createFeedback(formInfo.solution_name);
        }).value
      "
    >
      <n-input v-model:value="formInfo.solution_name" placeholder="解决方案" />
    </n-form-item>
    <n-form-item>
      <n-radio :default-checked="false" v-model:checked="createGitRepo">
        是否初始化 Git 仓库
      </n-radio>
    </n-form-item>
    <n-form-item>
      <div id="confirm-info">
        <span id="hint">
          解决方案将保存在路径的 {{ formInfo.solution_name }} 文件夹下
        </span>
        <n-button type="primary" @click="create">确认</n-button>
      </div>
    </n-form-item>
  </n-form>
</template>

<style scoped>
#hint {
  font-size: xx-small;
  color: #737373;
}

#confirm-info {
  display: flex;
  justify-content: space-between;
  width: 100%;
  align-items: flex-end;
}
</style>
