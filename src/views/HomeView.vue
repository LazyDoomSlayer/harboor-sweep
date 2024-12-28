<script lang="ts" setup>
import VirtualList from '@/components/virtual/VirtualList.vue';

import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import { EScrollBehavior } from '@/types/virtual-list.types';
import type { TPortProcessList } from '@/types';

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

const searchModel = ref<number>(100);
const virtualListRef = ref();
</script>

<template>
  <input
    type="number"
    v-model="searchModel"
    label="search"
    placeholder="search"
  />
  <button @click.left="getPorts">fetch again</button>
  <button @click.left="virtualListRef.goToElementWithIndex(searchModel)">
    Search
  </button>
  <button @click.left="virtualListRef.goToBottom">To Bottom</button>
  <button @click.left="virtualListRef.goToStart">To Top</button>

  <div style="height: 900px">
    <VirtualList
      :list="ports"
      ref="virtualListRef"
      :item-height="21"
      :scroll-behavior="EScrollBehavior.SMOOTH"
    >
      <template #item="{ item: port }">
        Port: {{ port.port }}, PID: {{ port.pid }}, Process:
        {{ port.process_name }}, Procs.Path: {{ port.process_path }}
      </template>
    </VirtualList>
  </div>
</template>
