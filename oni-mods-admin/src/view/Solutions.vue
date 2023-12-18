<script setup lang="ts">
import { useProjectStore } from '../store/project.store';
import {
  addNewProject,
  CsprojItem,
  getCsprojList,
  getLatestVersion,
  getConfigInfo,
  Project,
  ResultBody,
  StatusCode,
} from '../uitls/invokes';
import { ref } from 'vue';
import {
  MenuOption,
  NButton,
  NCard,
  NForm,
  NFormItem,
  NFormItemGi,
  NGrid,
  NInput,
  NList,
  NListItem,
  NModal,
  NSpin,
  useMessage,
} from 'naive-ui';
import router from '../router';

const projectStore = useProjectStore();
const solutionItem = projectStore.solutionItem;
const csproj = ref<CsprojItem[]>([]);
const buttonLoading = ref<boolean>(false);
const message = useMessage();
const spinShow = ref<boolean>(false);
const modalShow = ref<boolean>(false);
const refreshVersionSpinShow = ref<boolean>(false);
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
  projectStore.currentCsProjInfo = item;
  router.push('/csproj');
}

async function createProject() {
  buttonLoading.value = true;
  const entries = Object.entries(createProjectInfo.value.PropertyGroup);
  let flag = false;
  entries.forEach(([_, value]) => {
    if(value == ""){
      flag = true;
    }
  });
  if (flag) {
    message.warning("请先完成信息填写");
    buttonLoading.value = false;
    return;
  }
  projectStore.createProjectInfo.project_name =
    createProjectInfo.value.PropertyGroup.RootNamespace;
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

async function refreshVersion(){
  refreshVersionSpinShow.value = true;
  let result:ResultBody = await getLatestVersion();
  if (result.code !== StatusCode.SUCCESS){
    message.error(result.message);
  } else {
    createProjectInfo.value.PropertyGroup.LastWorkingBuild = Number.parseInt(result.message);
    message.info("版本已更新")
  }
  refreshVersionSpinShow.value = false;
}

async function getConfigVersion(){
  refreshVersionSpinShow.value = true;
  let result:ResultBody = await getConfigInfo();
  if (result.code !== StatusCode.SUCCESS){
    message.error(result.message);
  } else {
    let msg = JSON.parse(result.message)
    createProjectInfo.value.PropertyGroup.LastWorkingBuild = Number.parseInt(msg.latest_version);
  }
  refreshVersionSpinShow.value = false;
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
        @click="()=>{modalShow = !modalShow; getConfigVersion();} "
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
        <n-form label-width="auto" size="small">
          <n-grid :cols="24" :x-gap="6">
            <n-form-item-gi label="模组名称" span="12">
              <n-input
                v-model:value="createProjectInfo.PropertyGroup.AssemblyTitle"
                placeholder="可以是中文"
              />
            </n-form-item-gi>
            <n-form-item-gi label="根命名空间" span="12">
              <n-input
                v-model:value="createProjectInfo.PropertyGroup.RootNamespace"
                placeholder="需要是英文"
              />
            </n-form-item-gi>
          </n-grid>
          <n-grid :cols="24" :x-gap="6">
            <n-form-item-gi label="模组版本号" span="12">
              <n-input
                v-model:value="createProjectInfo.PropertyGroup.AssemblyVersion"
                placeholder="需要符合规范"
              />
            </n-form-item-gi>
            <n-form-item-gi label="文件版本号" span="12">
              <n-input
                v-model:value="createProjectInfo.PropertyGroup.FileVersion"
                placeholder="需要符合规范"
              />
            </n-form-item-gi>
          </n-grid>
          <n-form-item label="模组描述">
            <n-input
              v-model:value="createProjectInfo.PropertyGroup.Description"
              placeholder="尽量简短"
              type="textarea"
            />
          </n-form-item>
          <n-grid :cols="24" :x-gap="20">
            <n-form-item-gi label="最低支持版本" span="12">
              <n-spin style="width: 100%" :size="'small'" :show="refreshVersionSpinShow">
                <div id="latest-version">
                  <span>{{createProjectInfo.PropertyGroup.LastWorkingBuild}}</span>
                  <n-button
                    type="primary"
                    @click="refreshVersion"
                    :loading="buttonLoading"
                  >刷新</n-button>
                </div>
              </n-spin>
            </n-form-item-gi>
            <n-form-item-gi label="平台" span="12">
              <span>{{createProjectInfo.PropertyGroup.Platforms}}</span>
            </n-form-item-gi>
          </n-grid>
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
#latest-version{
  width: 100%;
  display: flex;
  justify-content: space-between;
}

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
