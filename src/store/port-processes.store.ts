import { defineStore } from 'pinia';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

import {
  ESorting,
  type TPortProcessItem,
  type TPortProcessList,
} from '@/types';
import { type IUsePortProcessesStoreState } from '@/types/store/port-processes.types';
import { computed, ref } from 'vue';

export const usePortProcessesStore = defineStore('port-processes', () => {
  const processes = ref<IUsePortProcessesStoreState['processes']>([]);
  const sorting = ref<IUsePortProcessesStoreState['sorting']>({
    key: 'port',
    direction: ESorting.ASCENDING,
  });
  const processFocused = ref<TPortProcessItem | null>(null);

  const getSortedProcesses = computed(() => {
    const { key, direction } = sorting.value;

    if (!key || direction === ESorting.NONE) {
      return processes.value;
    }

    return [...processes.value].sort((a, b) => {
      const valueA = a[key];
      const valueB = b[key];

      if (typeof valueA === 'string' && typeof valueB === 'string') {
        return (
          valueA.localeCompare(valueB) *
          (direction === ESorting.ASCENDING ? 1 : -1)
        );
      }

      if (typeof valueA === 'number' && typeof valueB === 'number') {
        return (valueA - valueB) * (direction === ESorting.ASCENDING ? 1 : -1);
      }

      if (typeof valueA === 'boolean' && typeof valueB === 'boolean') {
        return (
          (Number(valueA) - Number(valueB)) *
          (direction === ESorting.ASCENDING ? 1 : -1)
        );
      }

      return 0;
    });
  });

  function deduplicateById(list: TPortProcessList): TPortProcessList {
    const map = new Map<string, TPortProcessItem>();

    for (const item of list) {
      map.set(item.id, item);
    }

    return Array.from(map.values());
  }

  const fetchActivePortProcesses = async () => {
    try {
      const result = await invoke('fetch_ports');
      processes.value = deduplicateById(result as TPortProcessList);
    } catch (error) {
      console.error('Failed to fetch ports:', error);
    }
  };

  const startPortProcessesObserver = async () => {
    try {
      await invoke('update_interval', { newInterval: 2 });
      await invoke('start_monitoring');

      listen('ports_update', (event) => {
        processes.value = deduplicateById(event.payload as TPortProcessList);
      });
    } catch (error) {
      if (error === 'Monitoring is already running') {
        listen('ports_update', (event) => {
          processes.value = deduplicateById(event.payload as TPortProcessList);
        });
        return;
      }
      console.error(error);
    }
  };

  return {
    processes,
    processFocused,
    sorting,
    getSortedProcesses,
    fetchActivePortProcesses,
    startPortProcessesObserver,
  };
});
