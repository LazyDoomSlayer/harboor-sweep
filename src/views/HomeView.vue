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

// function focusOnTextField(elementReference: typeof BaseTextField): void {
//   try {
//     elementReference.focusField();
//   } catch (error) {
//     console.error(error);
//   }
// }
//
// function resetFiltration(): void {
//   pidModel.value = '';
//   portModel.value = '';
//   processNameModel.value = '';
//   processPathModel.value = '';
// }

const mainElementRef = useTemplateRef<HTMLElement>('mainElementRef');
const { height: mainElementHeight } = useElementSize(mainElementRef);
</script>

<template>
  <ConfirmKillingDialog />
  <ToastNotificationManager />
  <div style="flex-grow: 1; display: flex; flex-direction: column">
    <TheApplicationWindow ref="applicationWindowRef" />

    <main ref="mainElementRef" style="flex-grow: 1">
      <TheApplicationProcessSearchComponent v-model="searchModel" />
      <V2PortProccessesList
        v-if="mainElementHeight > 0"
        :available-height="mainElementHeight"
        :list="computedProcesses"
      />
      <TheApplicationProcessFooter />
    </main>
  </div>

  <!--    <div class="port-processes-filtration__wrapper">-->
  <!--      <div class="port-processes-filtration__filters">-->
  <!--        <BaseLabeledBox-->
  <!--          :active-color="boxActiveColor"-->
  <!--          :background-color="boxBackgroundColor"-->
  <!--          :color="boxColor"-->
  <!--          :is-active="!!pidModel.length"-->
  <!--          @click.left="focusOnTextField(pidTextFieldRef)"-->
  <!--        >-->
  <!--          <template #label>PID</template>-->
  <!--          <template #content>-->
  <!--            <BaseTextField-->
  <!--              ref="pidTextFieldRef"-->
  <!--              v-model="pidModel"-->
  <!--              placeholder="0001"-->
  <!--            />-->
  <!--          </template>-->
  <!--        </BaseLabeledBox>-->
  <!--        <BaseLabeledBox-->
  <!--          :active-color="boxActiveColor"-->
  <!--          :background-color="boxBackgroundColor"-->
  <!--          :color="boxColor"-->
  <!--          :is-active="!!portModel.length"-->
  <!--          @click.left="focusOnTextField(portTextFieldRef)"-->
  <!--        >-->
  <!--          <template #label>Port</template>-->
  <!--          <template #content>-->
  <!--            <BaseTextField-->
  <!--              ref="portTextFieldRef"-->
  <!--              v-model="portModel"-->
  <!--              placeholder="3000"-->
  <!--            />-->
  <!--          </template>-->
  <!--        </BaseLabeledBox>-->
  <!--        <BaseLabeledBox-->
  <!--          :active-color="boxActiveColor"-->
  <!--          :background-color="boxBackgroundColor"-->
  <!--          :color="boxColor"-->
  <!--          :is-active="!!processNameModel.length"-->
  <!--          @click.left="focusOnTextField(processNameTextFieldRef)"-->
  <!--        >-->
  <!--          <template #label>Process Name</template>-->
  <!--          <template #content>-->
  <!--            <BaseTextField-->
  <!--              ref="processNameTextFieldRef"-->
  <!--              v-model.trim="processNameModel"-->
  <!--              placeholder="node"-->
  <!--            />-->
  <!--          </template>-->
  <!--        </BaseLabeledBox>-->
  <!--        <BaseLabeledBox-->
  <!--          :active-color="boxActiveColor"-->
  <!--          :background-color="boxBackgroundColor"-->
  <!--          :color="boxColor"-->
  <!--          :is-active="!!processPathModel.length"-->
  <!--          @click.left="focusOnTextField(processPathTextFieldRef)"-->
  <!--        >-->
  <!--          <template #label>Process Path</template>-->
  <!--          <template #content>-->
  <!--            <BaseTextField-->
  <!--              ref="processPathTextFieldRef"-->
  <!--              v-model.trim="processPathModel"-->
  <!--              placeholder="/usr/bin/gnome-software"-->
  <!--            />-->
  <!--          </template>-->
  <!--        </BaseLabeledBox>-->
  <!--      </div>-->
  <!--      <BaseButton text="Reset Filtration" @left-clicked="resetFiltration" />-->
  <!--    </div>-->
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.port-processes-filtration {
  @include mixins.flex-display;
  flex-direction: row;
  justify-content: flex-start;
  align-content: center;
  position: relative;

  padding-bottom: 8px;
  border-bottom: 1px dashed #3e3e3e;

  &__wrapper {
    @include mixins.flex-display;
    flex-direction: row;
    justify-content: flex-start;
    align-content: center;
    position: relative;

    margin: 16px 6px 6px 6px;
  }

  &__filters {
    @include mixins.flex-display;
    gap: 6px;
    margin-right: 6px;
    flex-grow: 1;

    & > div {
      width: 25%;
    }
  }
}

main > section {
  //@include mixins.flex-display;
}
</style>
