import { defineStore } from 'pinia';
import { ref } from 'vue';
import { EApplicationCurrentView } from '@/types/store/application.types.ts';

export const useApplicationStore = defineStore('applicationStore', () => {
  const isApplicationFocused = ref<boolean>(true);
  const searchComponentOpen = ref<boolean>(false);
  const processFooterComponentOpen = ref<boolean>(false);
  const currentApplicationWindow = ref<EApplicationCurrentView>(
    EApplicationCurrentView.PORT_PROCESSES,
  );

  return {
    isApplicationFocused,
    searchComponentOpen,
    currentApplicationWindow,
    processFooterComponentOpen,
  };
});
