import { defineStore } from 'pinia';

import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import { ESorting, type TPortProcessList } from '@/types';
import {
  type IUsePortProcessesStoreState,
  EUsePortProcessesStoreActions,
  EUsePortProcessesStoreGetters,
} from '@/types/store/port-processes.types';

export const usePortProcessesStore = defineStore('port-processes', {
  state: (): IUsePortProcessesStoreState => ({
    processes: [],
    sorting: {
      key: 'port',
      direction: ESorting.ASCENDING,
    },
  }),
  getters: {
    [EUsePortProcessesStoreGetters.GET_SORTED_PROCESSES](
      state: IUsePortProcessesStoreState,
    ): IUsePortProcessesStoreState['processes'] {
      const { key, direction } = state.sorting;

      if (!key || direction === ESorting.NONE) {
        return state.processes;
      }

      return [...state.processes].sort((a, b) => {
        const valueA = a[key];
        const valueB = b[key];

        if (typeof valueA === 'string' && typeof valueB === 'string') {
          return (
            valueA.localeCompare(valueB) *
            (direction === ESorting.ASCENDING ? 1 : -1)
          );
        }

        if (typeof valueA === 'number' && typeof valueB === 'number') {
          return (
            (valueA - valueB) * (direction === ESorting.ASCENDING ? 1 : -1)
          );
        }

        return 0;
      });
    },
  },
  actions: {
    async [EUsePortProcessesStoreActions.FETCH_ACTIVE_PORT_PROCCESSES](): Promise<void> {
      try {
        const result: TPortProcessList = await invoke('fetch_ports');

        this.processes = result;
      } catch (error) {
        console.error('Failed to fetch ports:', error);
      }
    },
    async [EUsePortProcessesStoreActions.START_PORT_PROCCESSES_OBSERVER](): Promise<void> {
      try {
        await invoke('update_interval', { newInterval: 2 });
        await invoke('start_monitoring');

        listen('ports_update', (event) => {
          this.processes = event.payload as TPortProcessList;
        });
      } catch (error) {
        console.error(error);
      }
    },
  },
});
