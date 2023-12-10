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
  NLayout,
  NMenu,
  NScrollbar,
  NLayoutContent,
  MenuOption, NList, NListItem,
} from 'naive-ui';
import {ProjectOutlined as ProjIcon} from "@vicons/antd"
import router from '../router';
import {renderIcon} from "../uitls/menu";

const projectStore = useProjectStore();
const solutionItem = projectStore.solutionItem;
const csproj = ref<CsprojItem[]>([]);
const buttonLoading = ref<boolean>(false);
const message = useMessage();
const spinShow = ref<boolean>(false);
const modalShow = ref<boolean>(false);
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

const menuOption = ref<MenuOption[]>()


function handleUpdateValue (key: string, item: MenuOption) {
  message.info('[onUpdate:value]: ' + JSON.stringify(key))
  message.info('[onUpdate:value]: ' + JSON.stringify(item))
}

async function getCsprojListN() {
  spinShow.value = true;
  let result = await getCsprojList(solutionItem);
  if (result.code !== StatusCode.SUCCESS) {
    message.error('读取项目失败' + result.message);
  }
  csproj.value = await JSON.parse(result.message);
  let menuOptionsBuffer = [];
  for (let i = 0; i < csproj.value.length; i++){
    let newMenuOption:MenuOption = {
      key: i,
      label: csproj.value[i].name,
    }
    menuOptionsBuffer.push(newMenuOption);
  }
  menuOption.value = menuOptionsBuffer;
  spinShow.value = false;
}

async function back() {
  router.back();
}

function openSelectedCsproj(item:CsprojItem){
  console.log(item)
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
    <n-spin :show="spinShow" class="lists">
      <n-list hoverable clickable>
        <template v-for="item in csproj">
          <n-list-item
              class="solution-list-item"
              @click="openSelectedCsproj(item)"
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
    </n-spin>
  </div>
</template>

<style scoped>
#csproj-lists {
  padding: 10px;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}

.lists{
  margin-top: 40px;
  padding-bottom: 20px;
}

#option {
  display: flex;
  gap: 6px;
  width: 100%;
  padding-top: 10px;
  padding-left: 10px;
  padding-bottom: 10px;
  border-bottom: 1px solid #efefef;
  position: fixed;
  z-index: 9;
  background: #ffffff;
  top:0;
  left: 0;
}

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
  color: #858585;
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
</style>
