<template>
  <svg
    class="spinner"
    :width="size"
    :height="size"
    :viewBox="`0 0 ${size} ${size}`"
    xmlns="http://www.w3.org/2000/svg"
  >
    <circle
      class="path"
      fill="none"
      :stroke-width="strokeWidth"
      stroke-linecap="round"
      :cx="size / 2"
      :cy="size / 2"
      :r="(size - strokeWidth) / 2"
    ></circle>
  </svg>
</template>

<script setup lang="ts">
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

<style scoped lang="scss">
.spinner {
  animation: rotate 1s linear infinite;
}

@keyframes rotate {
  100% {
    transform: rotate(360deg);
  }
}

.path {
  stroke: #3498db;
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
    stroke: #3498db;
  }
  25% {
    stroke: #e74c3c;
  }
  50% {
    stroke: #f1c40f;
  }
  75% {
    stroke: #2ecc71;
  }
  100% {
    stroke: #3498db;
  }
}
</style>
