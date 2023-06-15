<script setup lang="ts">
import {
  NLayout,
  NLayoutContent,
  NLayoutHeader,
  NMessageProvider,
} from 'naive-ui';
</script>

<template>
  <n-layout class="full-height">
    <!--    <n-layout-header class="nav"></n-layout-header>-->
    <!--    <n-layout has-sider class="main">-->
    <!--      <n-layout-sider class="menu">-->
    <!--        <VerticalMenu/>-->
    <!--      </n-layout-sider>-->
    <!--      <n-layout-content class="view">-->
    <!--        <router-view></router-view>-->
    <!--      </n-layout-content>-->
    <!--    </n-layout>-->
    <n-layout class="full-height">
      <n-layout-content class="full-height view">
        <n-message-provider>
          <router-view class="full-height"></router-view>
        </n-message-provider>
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<style scoped>
.nav {
  padding: 0;
  border-bottom: 1px solid var(--n-border-color);
  height: var(--nav-height);
}

.menu {
  border-right: 1px solid var(--n-border-color);
}
</style>
