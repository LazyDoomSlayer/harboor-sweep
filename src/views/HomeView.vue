<script lang="ts" setup>
import PortProcessesTable from '@/components/PortProcessesTable.vue';
import ConfirmKillingDialog from '@/components/dialog/ConfirmKillingDialog.vue';

import { usePortProcessesStore } from '@/store/port-processes.store';
import { EUsePortProcessesStoreActions } from '@/types/store/port-processes.types';
import { storeToRefs } from 'pinia';
import { onMounted } from 'vue';

const portProcessesStore = usePortProcessesStore();
const { processes } = storeToRefs(portProcessesStore);

onMounted(async () => {
  await portProcessesStore[
    EUsePortProcessesStoreActions.GET_ACTIVE_PORT_PROCCESSES
  ]();
});
</script>

<template>
  <ConfirmKillingDialog />

  <PortProcessesTable :list="processes" />
</template>
