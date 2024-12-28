import { ref, type Ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { IKillProcessResponse } from '@/types';

interface IUseKillProcessResult {
  isLoading: Ref;
  kill: (pid: number) => Promise<IKillProcessResponse | null>;
}

export default function useKillProcess(): IUseKillProcessResult {
  const isLoading = ref<boolean>(false);

  async function kill(pid: number): Promise<IKillProcessResponse | null> {
    isLoading.value = true;

    try {
      return await invoke('kill_process', { pid });
    } catch (error) {
      console.error(error);
    } finally {
      isLoading.value = false;
    }

    return null;
  }

  return {
    isLoading,
    kill,
  };
}
