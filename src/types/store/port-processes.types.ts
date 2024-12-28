import type { TPortProcessList } from '@/types';

export interface IUsePortProcessesStoreState {
  processes: TPortProcessList;
}
export enum EUsePortProcessesStoreActions {
  GET_ACTIVE_PORT_PROCCESSES = 'GET_ACTIVE_PORT_PROCCESSES',
}
