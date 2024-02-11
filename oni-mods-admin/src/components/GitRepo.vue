<script setup lang="ts">
import { NEmpty, NButton } from 'naive-ui';
import { GitBranch } from '@vicons/ionicons5';
</script>

<template>
  <div id="repo-container">
    <n-empty description="没有仓库" style="padding-top: 15vh">
      <template #icon>
        <n-icon>
          <GitBranch />
        </n-icon>
      </template>
      <template #extra>
        <n-button size="small"> 新建 Git 仓库 </n-button>
      </template>
    </n-empty>
  </div>
</template>

<style scoped>
#repo-container {
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
