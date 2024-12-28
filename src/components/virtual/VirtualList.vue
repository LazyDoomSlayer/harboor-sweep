<script lang="ts" setup>
import { computed, type CSSProperties, ref } from 'vue';
import { EScrollBehavior, type IProps } from '@/types/virtual-list.types';

const componentProps = withDefaults(defineProps<IProps>(), {
  scrollBehavior: EScrollBehavior.AUTO,
});

const virtualListStyle = computed<CSSProperties>(() => {
  return {
    scrollBehavior: componentProps.scrollBehavior,
  };
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
  goToElementWithIndex(componentProps.list.length - 1);
}

defineExpose({
  virtualList,
  goToElementWithIndex,
  goToStart,
  goToBottom,
});
</script>

<template>
  <RecycleScroller
    ref="virtualList"
    :style="virtualListStyle"
    class="recycle-scroller"
    :items="componentProps.list"
    :item-size="componentProps.itemHeight"
    key-field="id"
    v-slot="{ item }"
  >
    <slot name="item" :item="item"></slot>
  </RecycleScroller>
</template>

<style lang="scss" scoped>
.recycle-scroller {
  height: 100%;
}
</style>
