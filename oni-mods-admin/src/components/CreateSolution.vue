<script setup lang="ts">
import {
  NForm,
  NFormItem,
  NInput,
  NButton,
  NRadio,
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
import { useProjectStore } from '../store/project.store';
import { ArrowLeftOutlined as Back } from '@vicons/antd';
import { useRouter } from 'vue-router';

const router = useRouter();

const formRef = ref<FormInst | null>(null);
const formInfo = ref<CreateProjectInfo>({
  root: '',
  project_name: '',
  solution_name: 'ONI-Mods',
});
const projectStore = useProjectStore();
const loadingVisible = ref<boolean>(false);
const message = useMessage();
const createGitRepo = ref<boolean>(false);
const reg: RegExp = /^[a-zA-Z0-9_-]+$/;
formInfo.value.root = projectStore.createProjectInfo.root;
function createStatus(value: string) {
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
  if (createStatus(formInfo.value.solution_name) === 'error') {
    message.warning('请正确填写信息');
    return;
  }
  loadingVisible.value = true;
  const result: ResultBody = await createProject(formInfo.value);
  loadingVisible.value = false;
  if (result.code === StatusCode.SUCCESS) {
    message.success('成功');
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
</script>

<template>
  <n-form
    label-placement="left"
    label-width="auto"
    ref="formRef"
    style="width: 60vw"
  >
    <n-form-item label="解决方案位置">
      <n-input
        v-model:value="formInfo.root"
        disabled
        placeholder="如果你能看到我，说明出错了"
      />
    </n-form-item>
    <n-form-item
      label="解决方案名称"
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
    <n-form-item class="flex flex-end center-offset">
      <n-button type="primary" @click="create">确认</n-button>
    </n-form-item>
  </n-form>
</template>

<style scoped></style>
