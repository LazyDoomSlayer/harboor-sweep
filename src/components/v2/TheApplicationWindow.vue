<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useApplicationStore } from '@/store/application.store.ts';
import { EApplicationCurrentView } from '@/types/store/application.types.ts';

const appWindow = getCurrentWindow();

const applicationStore = useApplicationStore();
</script>

<template>
  <div id="titlebar-wrapper" class="titlebar" data-tauri-drag-region>
    <div>
      <button
        id="titlebar-search"
        class="titlebar__button titlebar__button-search"
        @click.left="
          applicationStore.searchComponentOpen =
            !applicationStore.searchComponentOpen
        "
      >
        <span class="material-symbols-outlined"> search </span>
      </button>
    </div>

    <div class="titlebar__tab-wrapper">
      <button
        :class="{
          'is-active titlebar_tab-selected':
            applicationStore.currentApplicationWindow ===
            EApplicationCurrentView.PORT_PROCESSES,
        }"
        class="titlebar_tab"
        @click="
          applicationStore.currentApplicationWindow ===
          EApplicationCurrentView.PORT_PROCESSES
        "
      >
        <span class="material-symbols-rounded material-symbols-rounded__filled">
          play_arrow
        </span>

        Port Processes
      </button>
    </div>

    <div class="titlebar__actions">
      <button
        id="titlebar-minimize"
        class="titlebar__button titlebar__button-minimize"
        @click.left="appWindow.minimize()"
      >
        <span class="material-symbols-rounded"> remove </span>
      </button>
      <button
        id="titlebar-maximize"
        class="titlebar__button titlebar__button-maximize"
        @click.left="appWindow.toggleMaximize()"
      >
        <span class="material-symbols-rounded"> check_box_outline_blank </span>
      </button>
      <button
        id="titlebar-close"
        class="titlebar__button titlebar__button-close"
        @click.left="appWindow.close()"
      >
        <span class="material-symbols-rounded"> close </span>
      </button>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.titlebar {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  justify-content: space-between;
  align-items: center;

  position: fixed;
  top: 0;
  left: 0;
  right: 0;

  height: 40px;
  margin-bottom: 40px;
  padding: 0 8px;

  user-select: none;

  & > div {
    flex: 1;
  }

  &__button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    cursor: pointer;

    padding: 2px;
    z-index: 10;

    height: 22px;
    width: 22px;

    user-select: none;
    -webkit-user-select: none;

    border-radius: 6px;
    border: none;
    outline: 1px solid transparent;

    color: white;
    background-color: transparent;

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
      background-color: var(--main-bg-selected);
    }

    &:focus-visible {
      outline-color: var(--dialog-border);
    }
  }
}

.titlebar_tab {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  align-items: center;

  background: transparent;

  padding: 4px 6px;

  border: none;
  border-bottom: 2px solid transparent;
  border-radius: 6px;

  color: white;
  font-size: 14px;
  font-weight: 500;

  outline: 1px solid transparent;

  @include mixins.transition-all('medium');
  cursor: pointer;

  &-selected {
    cursor: default;
  }

  & > .material-symbols-rounded__filled {
    margin-right: 4px;
  }

  &:hover {
    background-color: rgba(255, 255, 255, 0.05);
  }

  &:focus-visible {
    outline-color: var(--dialog-border);
    outline-offset: 2px;
  }

  &.is-active {
    background-color: rgba(255, 255, 255, 0.1);
  }
}

.material-symbols-rounded__filled {
  font-variation-settings:
    'FILL' 1,
    'wght' 400,
    'GRAD' 0,
    'opsz' 16;
  font-size: 18px;
}

.titlebar__tab-wrapper {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  justify-content: center;
  align-items: center;
}

.titlebar__actions {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  justify-content: flex-end;
  align-items: center;
}
</style>
