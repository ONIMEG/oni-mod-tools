<script setup lang="ts">
import { useProjectStore } from '../store/project.store';
import { CsprojItem, getCsprojList, StatusCode } from '../uitls/invokes';
import { ref } from 'vue';
import {
  useMessage,
  NCard,
  NGrid,
  NFormItemGi,
  NForm,
  NInput,
  NButton,
  NSpin,
} from 'naive-ui';

const solutionItem = useProjectStore().solutionItem;
const csproj = ref<CsprojItem[]>([]);
const message = useMessage();
const spinShow = ref<boolean>(false);

async function getCsprojListN() {
  let result = await getCsprojList(solutionItem);
  if (result.code !== StatusCode.SUCCESS) {
    message.error('读取项目失败');
  }
  csproj.value = await JSON.parse(result.message);
}

getCsprojListN();
</script>

<template>
  <div id="csproj-lists">
    <n-spin :show="spinShow">
      <template v-for="(item, index) in csproj">
        <n-card :title="item.name" size="small">
          <n-form size="small" label-placement="top">
            <n-grid :cols="24" :x-gap="10">
              <n-form-item-gi :span="8" label="名称">
                <n-input v-model:value="csproj[index].name" />
              </n-form-item-gi>
              <n-form-item-gi :span="8" label="模组版本">
                <n-input />
              </n-form-item-gi>
              <n-form-item-gi :span="8" label="最低支持版本">
                <n-input />
              </n-form-item-gi>
              <n-form-item-gi :span="24" label="模组描述">
                <n-input />
              </n-form-item-gi>
              <n-form-item-gi :span="24">
                <div id="button-group">
                  <n-button type="default" size="small">保存修改</n-button>
                  <n-button type="success" size="small">生成</n-button>
                  <n-button type="error" size="small">删除</n-button>
                  <n-button type="info" size="small">发布</n-button>
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
</style>
