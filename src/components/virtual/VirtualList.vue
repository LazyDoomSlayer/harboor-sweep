<script lang="ts" setup>
// @ts-expect-error remove type issues

import { computed, type CSSProperties, ref } from 'vue';
import { EScrollBehavior, type IProps } from '@/types/virtual-list.types';
import { UseVirtualList } from '@vueuse/components';

const componentProps = withDefaults(defineProps<IProps>(), {
  scrollBehavior: EScrollBehavior.AUTO,
});

const virtualListStyle = computed<CSSProperties>(() => {
  return {
    scrollBehavior: componentProps.scrollBehavior,
  };
});
const computedList = computed(() => {
  console.log(' componentProps.list', componentProps.list);
  return componentProps.list;
});

const virtualList = ref();

function goToElementWithIndex(index: number): void {
  if (!virtualList.value) {
    throw new Error('could not find virtual list.');
  }

  try {
    virtualList.value.scrollToItem(index);
  } catch (error) {
    console.error(error);
  }
}

function goToStart(): void {
  goToElementWithIndex(0);
}

function goToBottom(): void {
  goToElementWithIndex(computedList.value.length - 1);
}

defineExpose({
  virtualList,
  goToElementWithIndex,
  goToStart,
  goToBottom,
});
</script>

<template>
  <UseVirtualList
    :list="computedList"
    :options="{ itemHeight: 32 }"
    height="100%"
  >
    <template #default="props">
      <slot :item="props.data" name="item"></slot>
    </template>
  </UseVirtualList>
</template>

<style lang="scss" scoped>
.recycle-scroller {
  height: 100%;
}
</style>
