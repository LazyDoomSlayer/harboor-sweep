<script lang="ts" setup>
import { computed, ref } from 'vue';
import { EScrollBehavior, type IProps } from '@/types/virtual-list.types.ts';
import { UseVirtualList } from '@vueuse/components';

const componentProps = withDefaults(defineProps<IProps>(), {
  scrollBehavior: EScrollBehavior.AUTO,
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
    :height="`${availableHeight}px`"
    :list="computedList"
    :options="{ itemHeight: componentProps.itemHeight }"
  >
    <template #default="props">
      <slot :item="props.data" name="item"></slot>
    </template>
  </UseVirtualList>
</template>

<style lang="scss" scoped></style>
