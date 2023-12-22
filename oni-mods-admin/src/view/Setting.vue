<script setup lang="ts">
import { NForm, NFormItem, NButton, NInput, useMessage } from 'naive-ui';
import { ref } from 'vue';
import { type ConfigInfo, getConfigInfo, StatusCode, updateConfigInfo } from '../uitls/invokes';
import router from '../router';
const configInfo = ref<ConfigInfo>({
  latest_version: 'test',
  github_token: 'test',
});

const message = useMessage();
async function getLocalConfig() {
  const result = await getConfigInfo();
  if (result.code == StatusCode.SUCCESS) {
    configInfo.value = JSON.parse(result.message);
    console.log(result.message)
    console.log(configInfo.value)
  } else {
    message.error('读取本地配置出错' + result.message);
  }
}
async function back() {
  const result = await updateConfigInfo(configInfo.value);
  if (result.code == StatusCode.SUCCESS) {
    message.success(result.message)
  }else {
    message.error(result.message)
  }
  router.back();
}
getLocalConfig();
</script>

<template>
  <n-form class="main" inline>
    <n-form-item label="GitHub Token">
      <n-input
        v-model:value="configInfo.github_token"
        placeholder="输入 GitHub token"
      />
    </n-form-item>
    <n-form-item label="缺氧最新版本号">
      <n-input v-model:value="configInfo.latest_version" /><n-button
        >刷新</n-button
      >
    </n-form-item>
  </n-form>
  <n-button @click="back">返回</n-button>
</template>

<style scoped>
.main{
  padding: 10px;
}

</style>
