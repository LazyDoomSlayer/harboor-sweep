<template>
  <div
    :style="{ height: `${computedVirtualListHeight}px` }"
    class="port-process-table"
  >
    <PortProcessListHeader />

    <BaseVirtualList
      v-if="computedVirtualListHeight > 0"
      ref="virtualListRef"
      :available-height="computedVirtualListHeight"
      :item-height="ITEM_HEIGHT"
      :list="props.list"
      :scroll-behavior="EScrollBehavior.SMOOTH"
      class="port-process-table__list"
    >
      <template #item="{ item }">
        <PortProcessesItem
          :max-item-height="ITEM_HEIGHT"
          :process="item as TPortProcessItem"
        />
      </template>
    </BaseVirtualList>
  </div>
</template>

<script lang="ts" setup>
import BaseVirtualList from '@/components/base/BaseVirtualList.vue';
import PortProcessesItem from '@/components/port-processes/PortProcessesItem.vue';
import PortProcessListHeader from '@/components/port-processes/PortProcessListHeader.vue';

import type { TPortProcessItem, TPortProcessList } from '@/types';
import { EScrollBehavior } from '@/types/virtual-list.types.ts';
import { computed, onMounted, ref } from 'vue';
import { useApplicationStore } from '@/store/application.store.ts';
import { usePortProcessesStore } from '@/store/port-processes.store.ts';

const ITEM_HEIGHT: number = 20;

const applicationStore = useApplicationStore();
const portProcessesStore = usePortProcessesStore();
const PROCESS_LIST_HEADER_HEIGHT: number = 40;
const searchComponentHeight = computed(() =>
  applicationStore.searchComponentOpen ? 32 : 0,
);
const processFooterHeight = computed(() =>
  applicationStore.processFooterComponentOpen ||
  portProcessesStore.processFocused
    ? 40
    : 0,
);

const computedVirtualListHeight = computed(
  () =>
    props.availableHeight -
    processFooterHeight.value -
    PROCESS_LIST_HEADER_HEIGHT -
    searchComponentHeight.value,
);

const props = defineProps<{
  list: TPortProcessList;
  availableHeight: number;
}>();

const virtualListRef = ref();
const applicationHeight = ref<number | null>(0);

const updateHeight = () => {
  applicationHeight.value = window.innerHeight;
};

onMounted(() => {
  updateHeight();
  window.addEventListener('resize', updateHeight);
});
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/mixins' as mixins;

.port-process-table {
  @include mixins.flex-display;
  @include mixins.flex-direction-column;

  &__list {
    @include mixins.flex-display;
    @include mixins.flex-direction-column;

    &-wrapper {
      margin-top: 12px;
      height: 100%;
    }
  }
}
</style>
