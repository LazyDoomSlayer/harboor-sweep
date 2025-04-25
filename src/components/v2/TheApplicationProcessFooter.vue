<script lang="ts" setup>
import { useApplicationStore } from '@/store/application.store.ts';
import { usePortProcessesStore } from '@/store/port-processes.store.ts';
import { toRaw } from 'vue';
import { useDialogsStore } from '@/store/dialogs.store.ts';
import { storeToRefs } from 'pinia';

const applicationStore = useApplicationStore();
const portProcessesStore = usePortProcessesStore();

const dialogStore = useDialogsStore();
const { confirmKillingDialog } = storeToRefs(dialogStore);

function killProcess(): void {
  if (!portProcessesStore.processFocused) {
    throw new Error('Port processes not found.');
  }

  confirmKillingDialog.value = {
    opened: true,
    process: toRaw(portProcessesStore.processFocused),
  };
}
</script>

<template>
  <footer
    v-if="
      applicationStore.processFooterComponentOpen ||
      portProcessesStore.processFocused
    "
    class="process-footer"
  >
    <button class="action-button" @click.left="killProcess">
      Kill Process
    </button>
  </footer>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.process-footer {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  justify-content: space-between;
  align-items: center;

  position: relative;

  height: 40px;
  padding: 0 8px;

  z-index: 4;
  user-select: none;

  border-top: 1px solid var(--main-divider-bg);
  background-color: var(--main-bg);
  color: var(--text-active);
}

.action-button {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  align-items: center;

  background-color: var(--system-negative-bg);

  height: 20px;

  border: none;
  border-radius: 6px;

  color: var(--system-negative-text);
  font-size: 12px;
  padding: 2px 4px;

  outline: 2px solid transparent;

  @include mixins.transition-all('medium');
  cursor: pointer;

  &-selected {
    cursor: default;
  }

  & > .material-symbols-rounded__filled {
    margin-right: 4px;
  }

  &:hover {
    background-color: var(--system-negative-bg-hovered);
  }

  &:focus-visible {
    outline-color: var(--system-negative-text);
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
</style>
