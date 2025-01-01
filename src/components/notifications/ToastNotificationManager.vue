<template>
  <div class="toast-manager">
    <div
      v-for="(toast, index) in toastNotifications"
      :key="toast.id"
      :style="{ top: `${index * 80}px` }"
      class="toast-wrapper"
    >
      <ToastNotificationItem
        :data="toast"
        :index="index"
        @remove-toast="handleRemovingToast"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import ToastNotificationItem from './ToastNotificationItem.vue';

import { useNotificationsStore } from '@/store/notifications.store';
import { storeToRefs } from 'pinia';
import { EUseNotificationsStoreActions } from '@/types/store/notifications.types';

const notificationStore = useNotificationsStore();
const { toastNotifications } = storeToRefs(notificationStore);

function handleRemovingToast(notification_id: string): void {
  try {
    notificationStore[EUseNotificationsStoreActions.REMOVE_TOAST_NOTIFICATION](
      notification_id,
    );
  } catch (error) {
    console.error(error);
  }
}
import { v4 as uuidv4 } from 'uuid';

notificationStore[EUseNotificationsStoreActions.ADD_TOAST_NOTICATION]({
  id: uuidv4(),
  title: `Success: Process with pid: ${191293923} , has been killed successfully.`,
});

notificationStore[EUseNotificationsStoreActions.ADD_TOAST_NOTICATION]({
  id: uuidv4(),
  title: '2nd',
  duration: 5_000,
});

notificationStore[EUseNotificationsStoreActions.ADD_TOAST_NOTICATION]({
  id: uuidv4(),
  title: '3nd',
  duration: 10_000,
});
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.toast-manager {
  position: absolute;
  z-index: 1000;
  @include mixins.flex-display;
  flex-direction: column;
  justify-content: flex-end;
  padding: 20px;
  height: calc(100dvh - 40px);

  pointer-events: none;

  top: 0;
  right: 0;
  gap: 10px;
}

.toast-wrapper {
  pointer-events: auto;
}
</style>
