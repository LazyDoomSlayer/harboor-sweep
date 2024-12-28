<template>
  <button
    @click.left="leftClicked($event)"
    @click.right="rightClicked($event)"
    :style="computedStyleObject"
    :class="{ 'disabled-state': componentProps.isDisabled }"
  >
    <slot name="prepend">
      <span v-if="componentProps.prependIcon">
        {{ componentProps.prependIcon }}
      </span>
    </slot>

    <CircleSpinner
      v-if="componentProps.isLoading"
      :stroke-width="4"
      :size="20"
    />

    <slot v-else name="content"> {{ text }} </slot>

    <slot name="append">
      <span v-if="componentProps.appendIcon">
        {{ componentProps.appendIcon }}
      </span>
    </slot>
  </button>
</template>

<script setup lang="ts">
import { computed, type CSSProperties } from 'vue';
import CircleSpinner from '../loaders/CircleSpinner.vue';

interface IProps {
  isDisabled?: boolean;
  isLoading?: boolean;
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
    cursor: componentProps.isDisabled ? 'default' : 'pointer',
  };
});

function leftClicked(event: MouseEvent) {
  if (componentProps.isLoading || componentProps.isDisabled) return;

  emit('leftClicked', event);
}

function rightClicked(event: MouseEvent) {
  if (componentProps.isLoading || componentProps.isDisabled) return;

  emit('rightClicked', event);
}
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

button {
  @include mixins.flex-display;
  justify-content: center;

  align-items: center;
}
</style>
