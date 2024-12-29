<script lang="ts" setup>
import PortProcessesTable from '@/components/PortProcessesTable.vue';
import ConfirmKillingDialog from '@/components/dialog/ConfirmKillingDialog.vue';

import { usePortProcessesStore } from '@/store/port-processes.store';
import {
  EUsePortProcessesStoreActions,
  EUsePortProcessesStoreGetters,
} from '@/types/store/port-processes.types';

import { computed, onMounted } from 'vue';

const portProcessesStore = usePortProcessesStore();

const computedProcesses = computed(
  () => portProcessesStore[EUsePortProcessesStoreGetters.GET_SORTED_PROCESSES],
);

onMounted(async () => {
  /*
  await portProcessesStore[
  EUsePortProcessesStoreActions.FETCH_ACTIVE_PORT_PROCCESSES
  ]();
  */
  await portProcessesStore[
    EUsePortProcessesStoreActions.START_PORT_PROCCESSES_OBSERVER
  ]();
});
</script>

<template>
  <ConfirmKillingDialog />

  <PortProcessesTable :list="computedProcesses" />
</template>
