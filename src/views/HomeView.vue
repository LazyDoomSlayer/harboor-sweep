<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import type { TPortProcessList } from '@/types';
import PortProcessesTable from '@/components/PortProcessesTable.vue';

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
  <PortProcessesTable :list="ports" />
</template>
