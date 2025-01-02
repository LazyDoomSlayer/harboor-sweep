<template>
  <div class="port-process-table">
    <PortProcessesTableHeader />
    <BaseLabeledBox
      class="port-process-table__list-wrapper"
      :background-color="boxBackgroundColor"
      :color="boxColor"
      :active-color="boxActiveColor"
      :is-active="false"
    >
      <template #label> Processes </template>
      <template #content>
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
      </template>
    </BaseLabeledBox>
  </div>
</template>

<script setup lang="ts">
import PortProcessesItem from '@/components/PortProcessesItem.vue';
import VirtualList from '@/components/virtual/VirtualList.vue';
import PortProcessesTableHeader from '@/components/PortProcessesTableHeader.vue';
import BaseLabeledBox from '@/components/base/BaseLabeledBox.vue';

import type { TPortProcessItem, TPortProcessList } from '@/types';
import { EScrollBehavior } from '@/types/virtual-list.types';
import { ref } from 'vue';

const ITEM_HEIGHT: number = 32;
const boxBackgroundColor = '#ffffff';
const boxColor = '#3e3e3e';
const boxActiveColor = '#fab700';

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
    }
  }
}
</style>
