<script setup lang="ts">
import { NLayout, NLayoutContent, NMessageProvider } from 'naive-ui';
</script>

<template>
  <n-layout class="full-height">
    <n-layout class="full-height">
      <n-layout-content class="full-height view">
        <n-message-provider>
          <router-view class="full-height" v-slot="{ Component }">
            <transition name="fade" mode="out-in">
              <component :is="Component" />
            </transition>
          </router-view>
        </n-message-provider>
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<style scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
