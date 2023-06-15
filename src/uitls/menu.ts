import { Component, h } from 'vue';
import { NIcon } from 'naive-ui';
import { RouterLink } from 'vue-router';

export function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

export function renderRouterLink(tag: string, name: string) {
  return () => h(RouterLink, { to: { name: name } }, { default: () => tag });
}
