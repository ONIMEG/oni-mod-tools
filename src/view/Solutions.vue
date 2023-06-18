<script setup lang="ts">
import { useProjectStore } from '../store/project.store';
import {
  CsprojItem,
  getCsprojList,
  addNewProject,
  StatusCode,
  Project,
} from '../uitls/invokes';
import { ref } from 'vue';
import {
  useMessage,
  NCard,
  NGrid,
  NFormItemGi,
  NFormItem,
  NForm,
  NInput,
  NButton,
  NSpin,
  NInputNumber,
  NModal,
} from 'naive-ui';
import router from '../router';

const projectStore = useProjectStore();
const solutionItem = projectStore.solutionItem;
const csproj = ref<CsprojItem[]>([]);
const buttonLoading = ref<boolean>(false);
const message = useMessage();
const spinShow = ref<boolean>(false);
const modalShow = ref<boolean>(true);
const createProjectInfo = ref<Project>({
  PropertyGroup: {
    AssemblyTitle: '',
    FileVersion: '1.0.0',
    RootNamespace: '',
    Description: '',
    AssemblyVersion: '1.0.0',
    LastWorkingBuild: 526233,
    Platforms: 'Vanilla;Mergedown',
  },
});

async function getCsprojListN() {
  spinShow.value = true;
  let result = await getCsprojList(solutionItem);
  if (result.code !== StatusCode.SUCCESS) {
    message.error('读取项目失败' + result.message);
  }
  csproj.value = await JSON.parse(result.message);
  spinShow.value = false;
}

async function back() {
  await router.back();
}

async function createProject() {
  buttonLoading.value = true;
  projectStore.createProjectInfo.project_name =
    createProjectInfo.value.PropertyGroup.AssemblyTitle;
  createProjectInfo.value.PropertyGroup.RootNamespace =
    createProjectInfo.value.PropertyGroup.AssemblyTitle;
  let result = await addNewProject(
    projectStore.createProjectInfo,
    createProjectInfo.value,
  );
  if (result.code === StatusCode.SUCCESS) {
    message.success('创建成功');
    modalShow.value = false;
    buttonLoading.value = false;
    return;
  }
  message.error('创建失败' + result.message);
  buttonLoading.value = false;
}

getCsprojListN();
</script>

<template>
  <div id="csproj-lists">
    <div id="option">
      <n-button
        type="primary"
        secondary
        size="small"
        @click="
          () => {
            modalShow = !modalShow;
          }
        "
        >新建项目</n-button
      >
      <n-button type="default" secondary @click="back" size="small"
        >返回</n-button
      >
    </div>
    <n-modal v-model:show="modalShow">
      <n-card
        style="width: 600px"
        title="新建项目"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
      >
        <n-form label-placement="left" label-width="auto" size="small">
          <n-form-item label="项目名称">
            <n-input
              v-model:value="createProjectInfo.PropertyGroup.AssemblyTitle"
            />
          </n-form-item>
          <n-form-item label="描述">
            <n-input
              v-model:value="createProjectInfo.PropertyGroup.Description"
            />
          </n-form-item>
          <n-form-item label="最低支持版本">
            <n-input-number
              v-model:value="createProjectInfo.PropertyGroup.LastWorkingBuild"
            />
          </n-form-item>
          <n-form-item class="flex flex-end">
            <n-button
              type="primary"
              style="margin-right: 6px"
              @click="createProject"
              :loading="buttonLoading"
              >确认</n-button
            >
            <n-button
              type="default"
              @click="
                () => {
                  modalShow = !modalShow;
                }
              "
              >取消</n-button
            >
          </n-form-item>
        </n-form>
      </n-card>
    </n-modal>
    <n-spin :show="spinShow">
      <template v-for="(item, index) in csproj">
        <n-card :title="item.name" size="small" style="margin-bottom: 10px">
          <n-form size="small" label-placement="top">
            <n-grid :cols="24" :x-gap="10">
              <n-form-item-gi :span="8" label="名称">
                <n-input
                  v-model:value="item.prop.PropertyGroup.AssemblyTitle"
                />
              </n-form-item-gi>
              <n-form-item-gi :span="8" label="模组版本">
                <n-input v-model:value="item.prop.PropertyGroup.FileVersion" />
              </n-form-item-gi>
              <n-form-item-gi :span="8" label="最低支持版本">
                <n-input-number
                  v-model:value="item.prop.PropertyGroup.LastWorkingBuild"
                />
              </n-form-item-gi>
              <n-form-item-gi :span="24" label="模组描述">
                <n-input v-model:value="item.prop.PropertyGroup.Description" />
              </n-form-item-gi>
              <n-form-item-gi :span="24">
                <div id="button-group">
                  <n-button type="default">保存修改</n-button>
                  <n-button type="success" disabled>生成</n-button>
                  <n-button type="error" disabled>删除</n-button>
                  <n-button type="info" disabled>发布</n-button>
                </div>
              </n-form-item-gi>
            </n-grid>
          </n-form>
        </n-card>
      </template>
    </n-spin>
  </div>
</template>

<style scoped>
#csproj-lists {
  padding: 10px;
  box-sizing: border-box;
}

#button-group {
  display: flex;
  gap: 6px;
  width: 100%;
  justify-content: flex-end;
}

#option {
  display: flex;
  gap: 6px;
  width: 100%;
  margin-bottom: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #efefef;
}
</style>
