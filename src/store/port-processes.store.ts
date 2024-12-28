import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import {
  type IUsePortProcessesStoreState,
  EUsePortProcessesStoreActions,
} from '@/types/store/port-processes.types';
import type { TPortProcessList } from '@/types';

export const usePortProcessesStore = defineStore('port-processes', {
  state: (): IUsePortProcessesStoreState => ({
    processes: [],
  }),
  getters: {},
  actions: {
    async [EUsePortProcessesStoreActions.GET_ACTIVE_PORT_PROCCESSES](): Promise<void> {
      try {
        const result: TPortProcessList = await invoke('fetch_ports');
        this.processes = result;
      } catch (error) {
        console.error('Failed to fetch ports:', error);
      }
    },
  },
});
