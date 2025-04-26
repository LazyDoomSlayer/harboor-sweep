<template>
  <svg
    :height="size"
    :viewBox="`0 0 ${size} ${size}`"
    :width="size"
    class="spinner"
    xmlns="http://www.w3.org/2000/svg"
  >
    <circle
      :cx="size / 2"
      :cy="size / 2"
      :r="(size - strokeWidth) / 2"
      :stroke-width="strokeWidth"
      class="path"
      fill="none"
      stroke-linecap="round"
    ></circle>
  </svg>
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    size?: number;
    strokeWidth?: number;
  }>(),
  {
    size: 50,
    strokeWidth: 5,
  },
);
</script>

<style lang="scss" scoped>
.spinner {
  animation: rotate 1s linear infinite;
}

@keyframes rotate {
  100% {
    transform: rotate(360deg);
  }
}

.path {
  stroke: var(--main-element-focused);
  stroke-dasharray: 150, 200;
  stroke-dashoffset: -10;
  animation:
    dash 1.5s ease-in-out infinite,
    color 6s ease-in-out infinite;
}

@keyframes dash {
  0% {
    stroke-dasharray: 1, 200;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 90, 200;
    stroke-dashoffset: -40;
  }
  100% {
    stroke-dasharray: 90, 200;
    stroke-dashoffset: -120;
  }
}

@keyframes color {
  0% {
    stroke: var(--main-element-focused);
  }
  25% {
    stroke: var(--main-input-icon);
  }
  50% {
    stroke: var(--text-main-input-label);
  }
  100% {
    stroke: var(--main-element-focused);
  }
}
</style>
