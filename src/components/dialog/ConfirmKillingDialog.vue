<template>
  <Teleport to="body" v-if="confirmKillingDialog.opened">
    <div class="dialog-overlay"></div>
    <div class="confirm-killing-dialog">
      <h2>Kill the following process ?</h2>
      <code>
        <pre v-text="confirmKillingDialog.process"></pre>
      </code>
      <div class="confirm-killing-dialog__buttons">
        <BaseButton
          text="CANCEL"
          @left-clicked="cancelKilling"
          class="flex-grow"
          :is-disabled="isLoading"
        />
        <BaseButton
          text="CONFIRM"
          @left-clicked="submitKilling"
          class="flex-grow"
          :is-loading="isLoading"
        />
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import BaseButton from '@/components/base/BaseButton.vue';

import { useDialogsStore } from '@/store/dialogs.store.ts';
import { storeToRefs } from 'pinia';

import useKillProcess from '@/composables/useKillProcess';
import { usePortProcessesStore } from '@/store/port-processes.store';
import { EUsePortProcessesStoreActions } from '@/types/store/port-processes.types';

const dialogStore = useDialogsStore();
const { confirmKillingDialog } = storeToRefs(dialogStore);

function closeAndRevertToDefaults(): void {
  confirmKillingDialog.value = {
    opened: false,
  };
}

const { isLoading, kill } = useKillProcess();

const portProcessesStore = usePortProcessesStore();

async function submitKilling(): Promise<void> {
  const pid = confirmKillingDialog.value.process?.pid;

  if (!pid) {
    throw new Error('Could not find process PID.');
  }

  try {
    const response = await kill(pid);

    if (response?.success) {
      await portProcessesStore[
        EUsePortProcessesStoreActions.GET_ACTIVE_PORT_PROCCESSES
      ]();

      closeAndRevertToDefaults();
    }
  } catch (error) {
    console.error(error);
  }
}
async function cancelKilling(): Promise<void> {
  closeAndRevertToDefaults();
}
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;
.dialog-overlay {
  height: 100%;
  width: 100%;
  position: absolute;
  background-color: #2e2e2e;
  opacity: 0.7;

  z-index: 9;

  left: 0;
  top: 0;
}

.confirm-killing-dialog {
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);

  z-index: 10;

  min-width: 400px;
  min-height: 200px;

  background-color: #e3e3e3;
  border-radius: 4px;
  border: 2px solid #1e1e1e;
  padding: 4px;

  &__buttons {
    @include mixins.flex-display;
    width: 100%;
    height: 30px;

    gap: 4px;
  }
}
</style>
