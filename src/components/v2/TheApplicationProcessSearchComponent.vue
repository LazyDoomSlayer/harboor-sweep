<script lang="ts" setup>
import { useApplicationStore } from '@/store/application.store.ts';
import { nextTick, onMounted, onUnmounted, useTemplateRef, watch } from 'vue';

const applicationStore = useApplicationStore();
const searchModel = defineModel({ default: '' });

const inputRef = useTemplateRef<HTMLInputElement>('inputRef');

watch(
  () => applicationStore.searchComponentOpen,
  async (isOpen) => {
    if (isOpen) {
      await nextTick();
      inputRef.value?.focus();
    }
  },
);

function handleKeydown(event: KeyboardEvent) {
  if (event.ctrlKey && event.key === 'f') {
    event.preventDefault();
    applicationStore.searchComponentOpen =
      !applicationStore.searchComponentOpen;
  }
}

watch(
  () => applicationStore.searchComponentOpen,
  (value) => {
    if (!value) {
      searchModel.value = '';
    }
  },
);

onMounted(() => {
  window.addEventListener('keydown', handleKeydown);
});

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeydown);
});
</script>

<template>
  <div
    v-if="applicationStore.searchComponentOpen"
    class="dropdown-content"
    @keydown.esc="applicationStore.searchComponentOpen = false"
  >
    <div class="search-bar">
      <span class="material-symbols-outlined search-icon"> search </span>

      <input
        ref="inputRef"
        v-model="searchModel"
        class="search-input"
        placeholder="Search PID, Port, Process name, Process path"
        type="text"
      />

      <span
        v-if="searchModel"
        class="material-symbols-rounded clear-icon"
        @click="searchModel = ''"
      >
        backspace
      </span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.dropdown-content {
  height: 32px;
  width: 100%;

  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  @include mixins.justify-content-center;
  align-items: center;
}

.search-bar {
  display: flex;
  align-items: center;
  max-height: 36px;
  width: 410px;

  background-color: var(--main-input-bg);
  border-radius: 6px;
  outline: 1px solid transparent;

  padding: 2px;
  color: var(--text-main-input-label);
  @include mixins.transition-all('medium');

  &:focus-within {
    outline: 2px solid var(--main-element-focused);
  }

  .search-icon {
    font-size: 18px;
    color: var(--main-input-icon);
    margin: 0 4px;
    @include mixins.transition-all('medium');
    cursor: pointer;

    &:hover {
      color: var(--text-main-input);
    }
  }

  .clear-icon {
    padding-right: 4px;
    font-size: 18px;
    cursor: pointer;
    @include mixins.transition-all('medium');

    &:hover {
      color: var(--text-main-input);
    }
  }

  .search-input {
    border: none;
    background: transparent;
    font-size: 14px;
    color: var(--text-main-input);
    width: 100%;
    outline: none;

    &:focus-visible .search-bar {
      outline-color: var(--main-element-focused);
    }

    &::placeholder {
      font-size: 12px;
      color: var(--text-main-input-label);
    }
  }
}
</style>
