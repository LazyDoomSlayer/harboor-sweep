import type { ESorting, TPortProcessItem, TPortProcessList } from '@/types';

export interface IUsePortProcessesStoreState {
  processes: TPortProcessList;
  sorting: {
    key: keyof TPortProcessItem;
    direction: ESorting;
  };
}
export enum EUsePortProcessesStoreActions {
  FETCH_ACTIVE_PORT_PROCCESSES = 'FETCH_ACTIVE_PORT_PROCCESSES',
}

export enum EUsePortProcessesStoreGetters {
  GET_SORTED_PROCESSES = 'GET_SORTED_PROCESSES',
}
