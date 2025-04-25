<template>
  <div class="port-process-table">
    <VirtualList
      ref="virtualListRef"
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

import type { TPortProcessItem, TPortProcessList } from '@/types';
import { EScrollBehavior } from '@/types/virtual-list.types';
import { ref } from 'vue';
import V2PortProcessesItem from '@/components/v2/V2PortProcessesItem.vue';

const ITEM_HEIGHT: number = 20;

// const boxBackgroundColor = getCssVariable('main-background-color');
// const boxColor = getCssVariable('base-label-border-passive-color');
// const boxActiveColor = getCssVariable('dialog-active-color');

const props = defineProps<{
  list: TPortProcessList;
}>();

const virtualListRef = ref();
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.port-process-table {
  @include mixins.flex-display;
  flex-direction: column;
  height: calc(100% - 54px - 12px);
  padding: 6px;

  &__list {
    @include mixins.flex-display;

    flex-direction: column;

    &-wrapper {
      margin-top: 12px;
      max-height: calc(100dvh - 68px - 58px - 38px - 12px) !important;
      height: 100%;
    }
  }
}
</style>
