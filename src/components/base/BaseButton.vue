<template>
  <button
    :class="{ 'disabled-state': componentProps.isDisabled }"
    :style="computedStyleObject"
    @click.left="leftClicked($event)"
    @click.right="rightClicked($event)"
  >
    <slot name="prepend">
      <span v-if="componentProps.prependIcon">
        {{ componentProps.prependIcon }}
      </span>
    </slot>

    <CircleSpinner
      v-if="componentProps.isLoading"
      :size="20"
      :stroke-width="4"
    />

    <slot v-else name="content"></slot>

    <slot name="append">
      <span v-if="componentProps.appendIcon">
        {{ componentProps.appendIcon }}
      </span>
    </slot>
  </button>
</template>

<script lang="ts" setup>
import { computed, type CSSProperties } from 'vue';
import CircleSpinner from '../loaders/CircleSpinner.vue';
import { getCssVariable } from '@/utils/theme-helper';

interface IProps {
  isDisabled?: boolean;
  isLoading?: boolean;
  prependIcon?: string;
  appendIcon?: string;

  backgroundColor?: string;
  textColor?: string;
}

const componentProps = withDefaults(defineProps<IProps>(), {});

const emit = defineEmits<{
  (e: 'leftClicked', event: MouseEvent): void;
  (e: 'rightClicked', event: MouseEvent): void;
}>();

const defaultTextColor = getCssVariable('base-label-border-passive-color');
const defaultBackgroundColor = getCssVariable('link-text-color');
const computedStyleObject = computed<CSSProperties>(() => {
  return {
    cursor: componentProps.isDisabled ? 'default' : 'pointer',
    backgroundColor: componentProps.backgroundColor ?? defaultBackgroundColor,
    color: componentProps.textColor ?? defaultTextColor,
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
  @include mixins.justify-content-center;
  @include mixins.align-items-center;

  border-radius: 6px;
  outline: 1px solid transparent;

  &:focus-within {
    outline: 2px solid var(--main-element-focused);
  }

  border: none;

  background-color: transparent;
  color: var(--text-active);
}
</style>
