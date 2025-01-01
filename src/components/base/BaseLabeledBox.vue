<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{
  color: string;
  activeColor: string;
  backgroundColor: string;

  isActive: boolean;

  labelPosition?: 'left' | 'center' | 'right';
}>();

const computedActiveColor = computed<string>(() => {
  if (props.isActive) {
    return props.activeColor;
  }

  return props.color;
});

const computedLabelStyle = computed(() => {
  switch (props.labelPosition) {
    case 'center':
      return { left: '50%', transform: 'translateX(-50%)' };
    case 'right':
      return { left: 'unset', right: '12px' };
    default:
      return { left: '12px' };
  }
});
</script>

<template>
  <div
    class="status-box"
    :style="{
      borderColor: computedActiveColor,
      backgroundColor: props.backgroundColor,
    }"
  >
    <!-- Label with slots -->
    <div
      class="status-box__label"
      :style="{
        backgroundColor: props.backgroundColor,
        borderColor: computedActiveColor,
        color: computedActiveColor,
        ...computedLabelStyle,
      }"
    >
      <slot name="labelPrefix"></slot>
      <slot name="label"></slot>
    </div>

    <!-- Content -->
    <div class="status-box__content">
      <slot name="content"></slot>
    </div>
  </div>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.status-box {
  position: relative;
  border: 2px solid;
  border-radius: 8px;
  padding: 12px;

  @include mixins.transition-all('medium');

  &__label {
    position: absolute;
    top: -10px;

    font-weight: bold;
    font-size: 14px;

    padding: 0 8px;

    border: 2px solid;
    border-radius: 4px;

    @include mixins.transition-all('medium');
  }

  &__content {
    @include mixins.flex-display;
    gap: 8px;

    font-size: 16px;
    height: 100%;

    color: white;
  }
}
</style>
