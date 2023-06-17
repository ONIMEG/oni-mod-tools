<script setup lang="ts">
import {
  NForm,
  NFormItem,
  NInput,
  NButton,
  NIcon,
  NSpin,
  FormInst,
  useMessage,
} from 'naive-ui';
import { computed, ref } from 'vue';
import {
  CreateProjectInfo,
  createProject,
  ResultBody,
  StatusCode,
} from '../uitls/invokes';
import { useProjectStore } from '../store/project.store';
import { ArrowLeftOutlined as Back } from '@vicons/antd';
import { useRouter } from 'vue-router';

const router = useRouter();

const formRef = ref<FormInst | null>(null);
const formInfo = ref<CreateProjectInfo>({
  root: '',
  project_name: 'NewMod',
  solution_name: 'ONI-Mods',
});
const projectStore = useProjectStore();
const loadingVisible = ref<boolean>(false);
const message = useMessage();
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
  if (
    createStatus(formInfo.value.project_name) === 'error' ||
    createStatus(formInfo.value.solution_name) === 'error'
  ) {
    message.warning('请正确填写信息');
    return;
  }
  loadingVisible.value = true;
  const result: ResultBody = await createProject(formInfo.value);
  loadingVisible.value = false;
  if (result.code === StatusCode.SUCCESS) {
    message.success('成功');
    return;
  }
  if (result.code === StatusCode.CONVERT_ERROR) {
    message.error('格式转换失败：' + result.message);
  } else {
    message.error(`项目创建失败：${result.message}`);
  }
  console.log(result.message);
}
</script>

<template>
  <n-spin class="form flex flex-all-center" :show="loadingVisible">
    <n-button
      id="back"
      @click="
        () => {
          router.back();
        }
      "
      secondary
    >
      <n-icon>
        <Back />
      </n-icon>
    </n-button>
    <h2 class="title">创建解决方案</h2>
    <n-form
      label-placement="left"
      label-width="auto"
      ref="formRef"
      style="width: 100%"
    >
      <n-form-item label="解决方案位置">
        <n-input
          v-model:value="formInfo.root"
          disabled
          placeholder="如果你能看到我，说明出错了"
        />
      </n-form-item>
      <n-form-item
        label="项目名"
        :validation-status="
          computed(() => {
            return createStatus(formInfo.project_name);
          }).value
        "
        :feedback="
          computed(() => {
            return createFeedback(formInfo.project_name);
          }).value
        "
      >
        <n-input v-model:value="formInfo.project_name" placeholder="模组名称" />
      </n-form-item>
      <n-form-item
        label="解决方案名"
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
        <n-input
          v-model:value="formInfo.solution_name"
          placeholder="解决方案"
        />
      </n-form-item>
      <n-form-item class="flex flex-end center-offset">
        <n-button type="primary" @click="create">确认</n-button>
      </n-form-item>
    </n-form>
  </n-spin>
</template>

<style scoped>
.form {
  padding: 1rem 2rem;
  height: 100vh;
}

.title {
  margin-bottom: 2rem;
}

#back {
  position: fixed;
  top: 1rem;
  left: 1rem;
}
</style>
