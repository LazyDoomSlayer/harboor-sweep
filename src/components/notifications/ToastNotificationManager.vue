<template>
  <div class="toast-manager">
    <div
      v-for="(toast, index) in toastNotifications"
      :key="toast.id"
      :style="{ top: `${index * 80}px` }"
      class="toast-manager__wrapper"
    >
      <ToastNotificationItem
        :data="toast"
        :index="index"
        @remove-toast="handleRemovingToast"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
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
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.toast-manager {
  @include mixins.flex-display;
  @include mixins.flex-direction-column;
  @include mixins.justify-content-flexEnd;

  position: absolute;
  top: 0;
  right: 0;
  z-index: 9;

  pointer-events: none;

  padding: 20px;
  gap: 10px;
  height: calc(100dvh - 40px);

  &__wrapper {
    pointer-events: auto;
    position: relative;
  }
}
</style>
