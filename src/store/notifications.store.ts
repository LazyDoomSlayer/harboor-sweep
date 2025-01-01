import {
  EUseNotificationsStoreActions,
  type IToastNotification,
  type IUseNotificationsStoreState,
} from '@/types/store/notifications.types';
import { defineStore } from 'pinia';

export const useNotificationsStore = defineStore('notifications', {
  state: (): IUseNotificationsStoreState => ({
    toastNotifications: [],
  }),
  getters: {},
  actions: {
    [EUseNotificationsStoreActions.ADD_TOAST_NOTICATION](
      notification: IToastNotification,
    ): void {
      this.toastNotifications.unshift(notification);
    },
    [EUseNotificationsStoreActions.REMOVE_TOAST_NOTIFICATION](
      notification_id: string,
    ): void {
      const index = this.toastNotifications.findIndex(
        (object) => object.id === notification_id,
      );

      if (index === -1) {
        console.error('Could not find toast nofication to remove.');
        return;
      }

      this.toastNotifications.splice(index, 1);
    },
  },
});
