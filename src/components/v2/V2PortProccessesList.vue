<template>
  <div
    :style="{ height: `${computedVirtualListHeight}px` }"
    class="port-process-table"
  >
    <!--    32px  -->
    <V2PortProcessListHeader />
    <span style="color: red; position: absolute; z-index: 312312">
      {{ computedVirtualListHeight }}
    </span>
    <VirtualList
      v-if="computedVirtualListHeight > 0"
      ref="virtualListRef"
      :available-height="computedVirtualListHeight"
      :item-height="ITEM_HEIGHT"
      :list="props.list"
      :scroll-behavior="EScrollBehavior.SMOOTH"
      class="port-process-table__list"
    >
      <template #item="{ item }">
        <V2PortProcessesItem
          :max-item-height="ITEM_HEIGHT"
          :process="item as TPortProcessItem"
        />
      </template>
    </VirtualList>
  </div>
</template>

<script lang="ts" setup>
import VirtualList from '@/components/virtual/VirtualList.vue';
import V2PortProcessesItem from '@/components/v2/V2PortProcessesItem.vue';
import V2PortProcessListHeader from '@/components/v2/V2PortProcessListHeader.vue';

import type { TPortProcessItem, TPortProcessList } from '@/types';
import { EScrollBehavior } from '@/types/virtual-list.types';
import { computed, onMounted, ref } from 'vue';
import { useApplicationStore } from '@/store/application.store.ts';
import { usePortProcessesStore } from '@/store/port-processes.store.ts';

const ITEM_HEIGHT: number = 20;

// const boxBackgroundColor = getCssVariable('main-background-color');
// const boxColor = getCssVariable('base-label-border-passive-color');
// const boxActiveColor = getCssVariable('dialog-active-color');

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
// const computedListStyle = computed<CSSProperties>(() => {
//   const spaceTaken =
//     PROCESS_LIST_HEADER_HEIGHT +
//     APPLICATION_WINDOW_HEIGHT +
//     processsFooterHeight.value +
//     searchComponentHeight.value;
//
//   return {
//     height: `calc(100vh - ${spaceTaken}px)`,
//   };
// });

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
@use '@/styles/abstracts/_mixins.scss' as mixins;

.port-process-table {
  @include mixins.flex-display;
  flex-direction: column;

  &__list {
    @include mixins.flex-display;
    flex-direction: column;

    &-wrapper {
      margin-top: 12px;
      height: 100%;
    }
  }
}
</style>
