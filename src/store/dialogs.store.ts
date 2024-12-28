import type { TPortProcessItem } from '@/types';
import { defineStore } from 'pinia';

interface IUseDialogStoreState {
  confirmKillingDialog: {
    opened: boolean;
    process?: TPortProcessItem;
  };
}

export const useDialogsStore = defineStore('dialogs', {
  state: (): IUseDialogStoreState => ({
    confirmKillingDialog: {
      opened: false,
    },
  }),
  getters: {},
  actions: {},
});
