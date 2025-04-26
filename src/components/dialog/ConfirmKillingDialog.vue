<template>
  <Teleport v-if="confirmKillingDialog.opened" to="body">
    <div class="confirm-killing-dialog__overlay"></div>
    <div class="confirm-killing-dialog">
      <h5 class="confirm-killing-dialog__title">
        Kill
        {{
          confirmKillingDialog.process?.process_name &&
          confirmKillingDialog.process?.process_name !== 'Unknown'
            ? confirmKillingDialog.process?.process_name
            : 'process'
        }}
        {{ confirmKillingDialog.process?.is_listener ? 'hosting' : 'using' }}
        port {{ confirmKillingDialog.process?.port }}
        ?
      </h5>
      <p class="confirm-killing-dialog__text">
        Ending this process may disrupt services using port
        {{ confirmKillingDialog.process?.port }}. Proceeding could result in
        data loss, network issues, or system instability.
      </p>
      <div class="confirm-killing-dialog__buttons">
        <BaseButton
          :is-disabled="isLoading"
          style="background: var(--cancel-button-bg); width: 50%"
          @left-clicked="cancelKilling"
        >
          <template #content>
            <span style="color: var(--text-active)"> CANCEL </span>
          </template>
        </BaseButton>
        <BaseButton
          :is-loading="isLoading"
          style="background: var(--system-negative-bg); width: 50%"
          @left-clicked="submitKilling"
        >
          <template #content>
            <span style="color: var(--system-negative-text)"> Kill </span>
          </template>
        </BaseButton>
      </div>
    </div>
  </Teleport>
</template>

<script lang="ts" setup>
import BaseButton from '@/components/base/BaseButton.vue';

import { useDialogsStore } from '@/store/dialogs.store.ts';
import { storeToRefs } from 'pinia';

import useKillProcess from '@/composables/useKillProcess';
import { usePortProcessesStore } from '@/store/port-processes.store';

import { v4 as uuidv4 } from 'uuid';
import { useNotificationsStore } from '@/store/notifications.store';
import { EUseNotificationsStoreActions } from '@/types/store/notifications.types';

const dialogStore = useDialogsStore();
const { confirmKillingDialog } = storeToRefs(dialogStore);

function closeAndRevertToDefaults(): void {
  confirmKillingDialog.value = {
    opened: false,
  };
}

const { isLoading, kill } = useKillProcess();

const portProcessesStore = usePortProcessesStore();
const notificationStore = useNotificationsStore();

async function submitKilling(): Promise<void> {
  const pid = confirmKillingDialog.value.process?.pid;

  if (!pid) {
    throw new Error('Could not find process PID.');
  }

  try {
    const response = await kill(pid);

    if (response?.success) {
      await portProcessesStore.fetchActivePortProcesses();

      closeAndRevertToDefaults();
      notificationStore[EUseNotificationsStoreActions.ADD_TOAST_NOTICATION]({
        id: uuidv4(),
        title: `Success: Process with pid: ${pid} , has been killed successfully.`,
      });
    } else {
      notificationStore[EUseNotificationsStoreActions.ADD_TOAST_NOTICATION]({
        id: uuidv4(),
        title: `Error: Could not kill process with pid: ${pid}.`,
      });
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

.confirm-killing-dialog {
  z-index: 6;
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);

  width: 360px;
  min-height: 120px;

  background-color: var(--main-dialog-bg);

  border-radius: 16px;
  padding: 8px 12px;

  &__overlay {
    z-index: 4;
    position: absolute;
    left: 0;
    top: 0;

    height: 100%;
    width: 100%;

    background-color: rgba(0, 0, 0, 0.4);
  }

  &__title {
    font-size: 16px;
    text-align: center;
    color: var(--text-main-dialog);

    margin: 4px 0;
  }

  &__text {
    color: var(--text-active);
    font-size: 12px;
    text-align: center;

    margin-top: 0;
  }

  &__buttons {
    @include mixins.flex-display;
    width: 100%;
    height: 30px;
    gap: 4px;

    background: transparent;
  }
}
</style>
