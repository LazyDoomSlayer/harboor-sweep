<script lang="ts" setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useApplicationStore } from '@/store/application.store.ts';
import { EApplicationCurrentView } from '@/types/store/application.types.ts';

const appWindow = getCurrentWindow();

const applicationStore = useApplicationStore();
</script>

<template>
  <header
    id="titlebar-wrapper"
    :class="[
      'titlebar',
      { 'titlebar--unfocused': !applicationStore.isApplicationFocused },
    ]"
    data-tauri-drag-region
  >
    <div>
      <button
        id="titlebar-search"
        class="titlebar__button titlebar__button--search"
        @click.left="
          applicationStore.searchComponentOpen =
            !applicationStore.searchComponentOpen
        "
      >
        <span class="material-symbols-outlined">search</span>
      </button>
    </div>

    <div class="spacer"></div>

    <div class="titlebar__tab-wrapper">
      <button
        :class="[
          'titlebar__tab',
          {
            'titlebar__tab--selected':
              applicationStore.currentApplicationWindow ===
              EApplicationCurrentView.PORT_PROCESSES,
          },
        ]"
        @click="
          applicationStore.currentApplicationWindow =
            EApplicationCurrentView.PORT_PROCESSES
        "
      >
        <span class="material-symbols-rounded material-symbols-rounded__filled">
          play_arrow
        </span>
        Port Processes
      </button>
    </div>

    <div class="spacer"></div>

    <div class="titlebar__actions">
      <button
        id="titlebar-minimize"
        class="titlebar__button titlebar__button--minimize"
        @click.left="appWindow.minimize()"
      >
        <span class="material-symbols-rounded">remove</span>
      </button>

      <button
        id="titlebar-maximize"
        class="titlebar__button titlebar__button--maximize"
        @click.left="appWindow.toggleMaximize()"
      >
        <span class="material-symbols-rounded">check_box_outline_blank</span>
      </button>

      <button
        id="titlebar-close"
        class="titlebar__button titlebar__button--close"
        @click.left="appWindow.close()"
      >
        <span class="material-symbols-rounded">close</span>
      </button>
    </div>
  </header>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/mixins' as mixins;

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
  user-select: none;

  &--unfocused {
    opacity: 0.5;
  }

  &__button {
    display: inline-flex;
    justify-content: center;
    align-items: center;

    width: 22px;
    height: 22px;
    padding: 2px;

    border: none;
    border-radius: 6px;
    outline: 1px solid transparent;

    background-color: transparent;
    color: white;

    cursor: pointer;
    user-select: none;
    z-index: 10;

    @include mixins.transition-all('medium');

    &--search span,
    &--minimize span,
    &--close span {
      font-size: 18px;
    }

    &--maximize span {
      font-size: 16px;
    }

    &:hover {
      background-color: var(--main-button-bg);
    }

    &:focus-visible {
      outline-color: var(--main-element-focused);
    }
  }

  &__tab {
    @include mixins.flex-display;
    @include mixins.flex-direction-row;
    align-items: center;

    background: transparent;
    border: none;
    border-radius: 6px;
    padding: 4px 6px;

    color: white;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;

    outline: 1px solid transparent;
    @include mixins.transition-all('medium');

    & > .material-symbols-rounded__filled {
      margin-right: 4px;
    }

    &:hover {
      background-color: rgba(255, 255, 255, 0.05);
    }

    &:focus-visible {
      outline-color: var(--main-element-focused);
      outline-offset: 2px;
    }

    &--selected {
      cursor: default;
      background-color: rgba(255, 255, 255, 0.1);
    }
  }

  &__tab-wrapper {
    @include mixins.flex-display;
    @include mixins.flex-direction-row;
    @include mixins.justify-content-center;
    align-items: center;

    margin-left: 44px;
  }

  &__actions {
    @include mixins.flex-display;
    @include mixins.flex-direction-row;
    @include mixins.justify-content-flexEnd;
    align-items: center;
  }
}

.material-symbols-rounded__filled {
  font-variation-settings:
    'FILL' 1,
    'wght' 400,
    'GRAD' 0,
    'opsz' 16;
  font-size: 16px;
}
</style>
