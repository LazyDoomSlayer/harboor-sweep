<script lang="ts" setup>
import { computed, CSSProperties, ref } from "vue";
// @ts-expect-error Somthing wrong with vue complier
import { EScrollBehavior } from "@/types/virtual-list.types";

interface Props {
  list: unknown[];
  scrollBehavior?: EScrollBehavior;
  itemHeight: number;
}

const props = withDefaults(defineProps<Props>(), {
  scrollBehavior: EScrollBehavior.AUTO,
});

const virtualListStyle = computed<CSSProperties>(() => {
  return {
    scrollBehavior: props.scrollBehavior,
  };
});

const virtualList = ref();

function goToElementWithIndex(index: number): void {
  if (!virtualList.value) {
    throw new Error("could not find virtual list.");
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
  goToElementWithIndex(props.list.length - 1);
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
    :items="props.list"
    :item-size="props.itemHeight"
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
