<script setup lang="ts">
import {
  NButton,
  NForm,
  NFormItem,
  NFormItemGi,
  NGrid,
  NInput,
  NSpin,
  useMessage,
} from 'naive-ui';
import { useProjectStore } from '../store/project.store';
import { ref } from 'vue';
import {
  CsprojItem,
  getLatestVersion,
  ResultBody,
  StatusCode,
} from '../uitls/invokes';

const projectStore = useProjectStore();
const projectItem =  ref<CsprojItem>(projectStore.currentCsProjInfo);
const projectInfo = projectItem.value.prop;
const buttonLoading = ref<boolean>(false);
const refreshVersionSpinShow = ref<boolean>(false);
const message = useMessage();

async function refreshVersion() {
  refreshVersionSpinShow.value = true;
  let result: ResultBody = await getLatestVersion();
  if (result.code !== StatusCode.SUCCESS) {
    message.error(result.message);
  } else {
    projectInfo.PropertyGroup.LastWorkingBuild = Number.parseInt(
      result.message,
    );
    message.info('版本已更新');
  }
  refreshVersionSpinShow.value = false;
}

function updateInfo() {
  console.log('p');
}
</script>

<template>
  <n-form label-width="auto" size="small">
    <n-grid :cols="24" :x-gap="6">
      <n-form-item-gi label="模组名称" span="12">
        <n-input
          v-model:value="projectInfo.PropertyGroup.AssemblyTitle"
          placeholder="可以是中文"
        />
      </n-form-item-gi>
      <n-form-item-gi label="根命名空间" span="12">
        <n-input
          v-model:value="projectInfo.PropertyGroup.RootNamespace"
          placeholder="需要是英文"
        />
      </n-form-item-gi>
    </n-grid>
    <n-grid :cols="24" :x-gap="6">
      <n-form-item-gi label="模组版本号" span="12">
        <n-input
          v-model:value="projectInfo.PropertyGroup.AssemblyVersion"
          placeholder="需要符合规范"
        />
      </n-form-item-gi>
      <n-form-item-gi label="文件版本号" span="12">
        <n-input
          v-model:value="projectInfo.PropertyGroup.FileVersion"
          placeholder="需要符合规范"
        />
      </n-form-item-gi>
    </n-grid>
    <n-form-item label="模组描述">
      <n-input
        v-model:value="projectInfo.PropertyGroup.Description"
        placeholder="尽量简短"
        type="textarea"
      />
    </n-form-item>
    <n-grid :cols="24" :x-gap="20">
      <n-form-item-gi label="最低支持版本" span="12">
        <n-spin
          style="width: 100%"
          :size="'small'"
          :show="refreshVersionSpinShow"
        >
          <div id="latest-version">
            <span>{{ projectInfo.PropertyGroup.LastWorkingBuild }}</span>
            <n-button
              type="primary"
              @click="refreshVersion"
              :loading="buttonLoading"
              >刷新</n-button
            >
          </div>
        </n-spin>
      </n-form-item-gi>
      <n-form-item-gi label="平台" span="12">
        <span>{{ projectInfo.PropertyGroup.Platforms }}</span>
      </n-form-item-gi>
    </n-grid>
    <n-form-item class="flex flex-end">
      <n-button
        type="primary"
        style="margin-right: 6px"
        @click="updateInfo"
        :loading="buttonLoading"
        >确认</n-button
      >
    </n-form-item>
  </n-form>
</template>

<style scoped></style>
