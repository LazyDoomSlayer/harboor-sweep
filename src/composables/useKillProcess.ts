import { ref, type Ref } from 'vue';

interface IUseKillProcessResult {
  isLoading: Ref;
  kill: () => Promise<void>;
}

export default function useKillProcess(): IUseKillProcessResult {
  const isLoading = ref<boolean>(false);

  async function kill(): Promise<void> {
    isLoading.value = true;
    try {
      console.log('do something from here');
    } catch (error) {
      console.error(error);
    } finally {
      //isLoading.value = false;
    }
  }

  return {
    isLoading,
    kill,
  };
}
