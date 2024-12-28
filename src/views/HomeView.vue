<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import type { TPortProcessList } from '@/types';
import PortProcessesTable from '@/components/PortProcessesTable.vue';
import ConfirmKillingDialog from '@/components/dialog/ConfirmKillingDialog.vue';

const ports = ref<TPortProcessList>([]);

async function getPorts(): Promise<void> {
  try {
    const result: TPortProcessList = await invoke('fetch_ports');
    ports.value = result;
    console.log('Fetched ports:', ports.value);
  } catch (error) {
    console.error('Failed to fetch ports:', error);
  }
}

onMounted(() => {
  getPorts();
});
</script>

<template>
  <ConfirmKillingDialog />

  <PortProcessesTable :list="ports" />
</template>
