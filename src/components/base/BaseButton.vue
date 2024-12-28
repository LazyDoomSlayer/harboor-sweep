<template>
  <button
    @click.left="emit('leftClicked', $event)"
    @click.right="emit('rightClicked', $event)"
    :style="computedStyleObject"
  >
    <slot name="prepend">
      <span v-if="componentProps.prependIcon">
        {{ componentProps.prependIcon }}
      </span>
    </slot>
    <slot name="content"> {{ text }} </slot>
    <slot name="append">
      <span v-if="componentProps.appendIcon">
        {{ componentProps.appendIcon }}
      </span>
    </slot>
  </button>
</template>

<script setup lang="ts">
import { computed, type CSSProperties } from 'vue';

interface IProps {
  isDisabled?: boolean;
  prependIcon?: string;
  appendIcon?: string;
  text: string;
}

const componentProps = withDefaults(defineProps<IProps>(), {});

const emit = defineEmits<{
  (e: 'leftClicked', event: MouseEvent): void;
  (e: 'rightClicked', event: MouseEvent): void;
}>();

const computedStyleObject = computed<CSSProperties>(() => {
  return {
    cursor: componentProps.isDisabled ? 'disabled' : 'pointer',
  };
});
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

button {
  @include mixins.flex-display;
  align-items: center;
}
</style>
