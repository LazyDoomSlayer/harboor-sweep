<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window';

const appWindow = getCurrentWindow();
</script>

<template>
  <div id="titlebar-wrapper" class="titlebar" data-tauri-drag-region>
    <div id="titlebar-search" class="titlebar__button titlebar__button-search">
      <span class="material-symbols-outlined"> search </span>
    </div>
    <div class="spacer"></div>

    <div>
      <button>here</button>
    </div>

    <div class="spacer"></div>
    <div
      id="titlebar-minimize"
      class="titlebar__button titlebar__button-minimize"
      @click.left="appWindow.minimize()"
    >
      <span class="material-symbols-rounded"> remove </span>
    </div>
    <div
      id="titlebar-maximize"
      class="titlebar__button titlebar__button-maximize"
      @click.left="appWindow.toggleMaximize()"
    >
      <span class="material-symbols-rounded"> check_box_outline_blank </span>
    </div>
    <div
      id="titlebar-close"
      class="titlebar__button titlebar__button-close"
      @click.left="appWindow.close()"
    >
      <span class="material-symbols-rounded"> close </span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.titlebar {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  @include mixins.justify-content-space-between;
  align-items: center;

  position: fixed;
  top: 0;
  left: 0;
  right: 0;

  height: 40px;
  padding: 0 8px;
  border-bottom: 1px red solid;
  color: white;
  user-select: none;

  &__button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;

    padding: 2px;
    user-select: none;
    -webkit-user-select: none;

    border-radius: 6px;
    z-index: 10;
    @include mixins.transition-all('medium');

    &-search > span,
    &-minimize > span,
    &-close > span {
      font-size: 18px !important;
    }

    &-maximize > span {
      font-size: 16px !important;
    }

    &:hover {
      background: var(--main-bg-selected);
    }
  }
}
</style>
