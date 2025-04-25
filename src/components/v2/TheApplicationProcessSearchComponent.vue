<script lang="ts" setup>
import { useApplicationStore } from '@/store/application.store.ts';

const applicationStore = useApplicationStore();
</script>

<template>
  <div v-if="applicationStore.searchComponentOpen" class="dropdown-content">
    <div class="search-bar">
      <span class="material-symbols-outlined search-icon"> search </span>
      <input
        class="search-input"
        placeholder="Search PID, Port, Process name, Process path"
        type="text"
      />
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
  width: 400px;

  background-color: var(--main-input-bg);
  border-radius: 6px;
  padding: 2px;
  color: var(--text-main-input-label);
  @include mixins.transition-all('medium');

  &:focus-visible {
    color: var(--text-main-input);
    outline-color: var(--main-element-focused);
  }

  .search-icon {
    font-size: 18px;
    color: var(--main-input-icon);
    margin: 0 4px;
    @include mixins.transition-all('medium');

    &:hover {
      color: var(--text-main-input);
    }
  }

  .search-input {
    border: none;
    outline: none;
    background: transparent;
    font-size: 14px;
    color: var(--text-main-input);
    width: 100%;

    &::placeholder {
      color: var(--text-main-input-label);
    }
  }
}
</style>
