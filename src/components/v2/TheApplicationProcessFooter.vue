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
    :class="[
      'process-footer',
      { 'process-footer--unfocused': !applicationStore.isApplicationFocused },
    ]"
  >
    <button class="process-footer__button" @click.left="killProcess">
      Kill Process
    </button>
  </footer>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.process-footer {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  @include mixins.justify-content-space-between;
  align-items: center;

  position: relative;
  z-index: 4;
  user-select: none;

  height: 40px;
  padding: 0 8px;

  background-color: var(--main-bg);
  color: var(--text-active);
  border-top: 1px solid var(--main-divider-bg);

  &--unfocused {
    opacity: 0.5;
  }

  &__button {
    @include mixins.flex-display;
    @include mixins.flex-direction-row;
    align-items: center;

    background-color: var(--system-negative-bg);
    color: var(--system-negative-text);

    height: 20px;
    padding: 4px 8px;
    font-size: 10px;
    font-weight: bold;

    border: none;
    border-radius: 6px;
    outline: 2px solid transparent;

    cursor: pointer;
    @include mixins.transition-all('medium');

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

    & > .material-symbols-rounded__filled {
      margin-right: 4px;
    }
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
