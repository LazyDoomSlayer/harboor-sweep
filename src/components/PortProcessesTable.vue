<template>
  <div class="port-process-table">
    <PortProcessesTableHeader />
    <VirtualList
      :list="props.list"
      ref="virtualListRef"
      class="port-process-table__list"
      :item-height="ITEM_HEIGHT"
      :scroll-behavior="EScrollBehavior.SMOOTH"
    >
      <template #item="{ item }">
        <PortProcessesItem
          :max-item-height="ITEM_HEIGHT"
          :process="item as TPortProcessItem"
        />
      </template>
    </VirtualList>
  </div>
</template>

<script setup lang="ts">
import PortProcessesItem from '@/components/PortProcessesItem.vue';
import VirtualList from '@/components/virtual/VirtualList.vue';

import type { TPortProcessItem, TPortProcessList } from '@/types';
import { EScrollBehavior } from '@/types/virtual-list.types';
import { ref } from 'vue';
import PortProcessesTableHeader from './PortProcessesTableHeader.vue';

const ITEM_HEIGHT: number = 32;
const props = defineProps<{
  list: TPortProcessList;
}>();

const virtualListRef = ref();
</script>

<style lang="scss" scoped>
.port-process-table {
  padding: 6px;

  &__list {
    margin-top: 4px;
    border: 2px solid #3e3e3e;
    padding: 4px;
    border-radius: 4px;
  }
}
</style>
