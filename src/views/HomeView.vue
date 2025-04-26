<script lang="ts" setup>
import ConfirmKillingDialog from '@/components/dialog/ConfirmKillingDialog.vue';
import ToastNotificationManager from '@/components/notifications/ToastNotificationManager.vue';
import TheApplicationWindow from '@/components/v2/TheApplicationWindow.vue';
import TheApplicationProcessSearchComponent from '@/components/v2/TheApplicationProcessSearchComponent.vue';
import TheApplicationProcessFooter from '@/components/v2/TheApplicationProcessFooter.vue';
import V2PortProccessesList from '@/components/v2/V2PortProccessesList.vue';

import { computed, onMounted, ref, useTemplateRef } from 'vue';

import { usePortProcessesStore } from '@/store/port-processes.store';
import type { TPortProcessItem, TPortProcessList } from '@/types';
import { useElementSize } from '@vueuse/core';

const portProcessesStore = usePortProcessesStore();

onMounted(async () => {
  await portProcessesStore.startPortProcessesObserver();
});

const searchModel = ref<string>('');

const computedProcesses = computed(() => {
  const processes: TPortProcessList = portProcessesStore.getSortedProcesses;
  const searchValue = searchModel.value.toLowerCase().trim();

  if (!searchValue) return processes;

  return processes.filter((process: TPortProcessItem) => {
    return (
      process.pid.toString().includes(searchValue) ||
      process.port.toString().includes(searchValue) ||
      process.process_name.toLowerCase().includes(searchValue) ||
      process.process_path.toLowerCase().includes(searchValue)
    );
  });
});

const mainElementRef = useTemplateRef<HTMLElement>('mainElementRef');
const { height: mainElementHeight } = useElementSize(mainElementRef);
</script>

<template>
  <!--  NEEDS REFACTOR-->
  <ConfirmKillingDialog />
  <ToastNotificationManager />

  <div class="port-processes">
    <TheApplicationWindow ref="applicationWindowRef" />

    <main ref="mainElementRef" class="port-processes__content">
      <TheApplicationProcessSearchComponent v-model="searchModel" />
      <V2PortProccessesList
        v-if="mainElementHeight > 0"
        :available-height="mainElementHeight"
        :list="computedProcesses"
      />
      <TheApplicationProcessFooter />
    </main>
  </div>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.port-processes {
  @include mixins.flex-display;
  @include mixins.flex-direction-column;
  @include mixins.justify-content-flexStart;
  @include mixins.align-content-center;

  position: relative;
  flex-grow: 1;

  &__content {
    flex-grow: 1;
  }
}
</style>
