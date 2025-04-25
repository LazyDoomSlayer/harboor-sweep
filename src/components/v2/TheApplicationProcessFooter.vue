<script lang="ts" setup>
import { useApplicationStore } from '@/store/application.store.ts';
import { usePortProcessesStore } from '@/store/port-processes.store.ts';

const applicationStore = useApplicationStore();
const portProcessesStore = usePortProcessesStore();
</script>

<template>
  <footer
    v-if="
      applicationStore.processFooterComponentOpen ||
      portProcessesStore.processFocused
    "
    class="process-footer"
  >
    footer here
  </footer>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.process-footer {
  @include mixins.flex-display;
  @include mixins.flex-direction-row;
  justify-content: space-between;
  align-items: center;

  position: fixed;

  bottom: 0;
  left: 0;
  right: 0;

  height: 40px;
  padding: 0 8px;

  z-index: 100;
  user-select: none;
}
</style>
