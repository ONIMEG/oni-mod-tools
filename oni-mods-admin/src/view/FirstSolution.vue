<script setup lang="ts">
import CreateSolution from '../components/CreateSolution.vue';
</script>

<template>
  <div id="first-solution-container">
    <span>创建解决方案</span>
    <CreateSolution />
  </div>
</template>

<style scoped>
#first-solution-container {
  height: 100%;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  gap: 40px;
}
</style>
