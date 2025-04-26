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
    class="search-dropdown"
    @keydown.esc="applicationStore.searchComponentOpen = false"
  >
    <div class="search-dropdown__bar">
      <span class="material-symbols-outlined search-dropdown__icon"
        >search</span
      >

      <input
        ref="inputRef"
        v-model="searchModel"
        class="search-dropdown__input"
        placeholder="Search PID, Port, Process name, Process path"
        type="text"
      />

      <span
        v-if="searchModel"
        class="material-symbols-rounded search-dropdown__clear"
        @click="searchModel = ''"
      >
        backspace
      </span>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/mixins' as mixins;

.search-dropdown {
  width: 100%;
  height: 32px;

  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  @include mixins.justify-content-center;
  align-items: center;

  &__bar {
    @include mixins.flex-display;
    @include mixins.flex-direction-row;
    align-items: center;
    width: 410px;
    max-height: 36px;

    background-color: var(--main-input-bg);
    border-radius: 6px;
    outline: 1px solid transparent;
    padding: 2px;

    color: var(--text-main-input-label);
    @include mixins.transition-all('medium');

    &:focus-within {
      outline: 2px solid var(--main-element-focused);
    }
  }

  &__icon {
    font-size: 18px;
    color: var(--main-input-icon);
    margin: 0 4px;
    cursor: pointer;
    @include mixins.transition-all('medium');

    &:hover {
      color: var(--text-main-input);
    }
  }

  &__clear {
    font-size: 18px;
    padding-right: 4px;
    cursor: pointer;
    @include mixins.transition-all('medium');

    &:hover {
      color: var(--text-main-input);
    }
  }

  &__input {
    flex: 1;
    border: none;
    background: transparent;
    font-size: 14px;
    color: var(--text-main-input);
    outline: none;

    &::placeholder {
      font-size: 12px;
      color: var(--text-main-input-label);
    }
  }
}
</style>
