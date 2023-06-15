<script setup lang="ts">
import { MenuOption, NMenu } from 'naive-ui';
import {
  ControlOutlined as SolutionList,
  ProjectOutlined as SolutionItem,
} from '@vicons/antd';
import { renderRouterLink, renderIcon } from '../uitls/menu';

const solutionItem: MenuOption[] = [
  {
    label: renderRouterLink('ONIMods', 'main'),
    key: 'oni-mods',
    icon: renderIcon(SolutionItem),
  },
];

const menuOptions: MenuOption[] = [
  {
    label: renderRouterLink('其他', 'main'),
    key: 'main',
    icon: renderIcon(SolutionItem),
  },
  {
    label: renderRouterLink('解决方案', 'main'),
    key: 'solution-list',
    icon: renderIcon(SolutionList),
    children: solutionItem,
  },
];
</script>

<template>
  <n-menu
    :inverted="false"
    :collapsed-width="64"
    :collapsed-icon-size="22"
    :options="menuOptions"
    :default-value="'solution-list'"
    :default-expanded-keys="['solution-list']"
  />
</template>

<style scoped></style>
