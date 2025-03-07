<script lang="ts" setup>
import PortProcessesTable from '@/components/PortProcessesTable.vue';
import ConfirmKillingDialog from '@/components/dialog/ConfirmKillingDialog.vue';
import ToastNotificationManager from '@/components/notifications/ToastNotificationManager.vue';
import BaseTextField from '@/components/base/BaseTextField.vue';
import BaseLabeledBox from '@/components/base/BaseLabeledBox.vue';
import BaseButton from '@/components/base/BaseButton.vue';

import { computed, onMounted, ref } from 'vue';

import { usePortProcessesStore } from '@/store/port-processes.store';
import {
  EUsePortProcessesStoreActions,
  EUsePortProcessesStoreGetters,
} from '@/types/store/port-processes.types';
import type { TPortProcessItem, TPortProcessList } from '@/types';

import { getCssVariable } from '@/utils/theme-helper';

const portProcessesStore = usePortProcessesStore();

onMounted(async () => {
  await portProcessesStore[
    EUsePortProcessesStoreActions.START_PORT_PROCCESSES_OBSERVER
  ]();
});

const boxBackgroundColor = getCssVariable('main-background-color');
const boxColor = getCssVariable('base-label-border-passive-color');
const boxActiveColor = getCssVariable('dialog-active-color');

const pidModel = ref<string>('');
const pidTextFieldRef = ref();

const portModel = ref<string>('');
const portTextFieldRef = ref();

const processNameModel = ref<string>('');
const processNameTextFieldRef = ref();

const processPathModel = ref<string>('');
const processPathTextFieldRef = ref();

const computedProcesses = computed(() => {
  const processes: TPortProcessList =
    portProcessesStore[EUsePortProcessesStoreGetters.GET_SORTED_PROCESSES];

  return processes.filter((process: TPortProcessItem) => {
    const pidMatch = process.pid
      .toString()
      .startsWith(pidModel.value.toString());

    const portMatch = process.port.toString().startsWith(portModel.value);

    const processNameMatch = process.process_name
      .toLowerCase()
      .startsWith(processNameModel.value.toLowerCase());

    const processPathMatch = process.process_path
      .toLowerCase()
      .startsWith(processPathModel.value.toLowerCase());

    return pidMatch && portMatch && processNameMatch && processPathMatch;
  });
});

function focusOnTextField(elementReference: typeof BaseTextField): void {
  try {
    elementReference.focusField();
  } catch (error) {
    console.error(error);
  }
}

function resetFiltration(): void {
  pidModel.value = '';
  portModel.value = '';
  processNameModel.value = '';
  processPathModel.value = '';
}
</script>

<template>
  <ConfirmKillingDialog />
  <ToastNotificationManager />

  <section style="display: flex; flex-direction: column; height: 100%">
    <div class="port-processes-filtration__wrapper">
      <div class="port-processes-filtration__filters">
        <BaseLabeledBox
          :background-color="boxBackgroundColor"
          :color="boxColor"
          :active-color="boxActiveColor"
          :is-active="!!pidModel.length"
          @click.left="focusOnTextField(pidTextFieldRef)"
        >
          <template #label>PID</template>
          <template #content>
            <BaseTextField
              v-model="pidModel"
              placeholder="0001"
              ref="pidTextFieldRef"
            />
          </template>
        </BaseLabeledBox>
        <BaseLabeledBox
          :background-color="boxBackgroundColor"
          :color="boxColor"
          :active-color="boxActiveColor"
          :is-active="!!portModel.length"
          @click.left="focusOnTextField(portTextFieldRef)"
        >
          <template #label>Port</template>
          <template #content>
            <BaseTextField
              v-model="portModel"
              placeholder="3000"
              ref="portTextFieldRef"
            />
          </template>
        </BaseLabeledBox>
        <BaseLabeledBox
          :background-color="boxBackgroundColor"
          :color="boxColor"
          :active-color="boxActiveColor"
          :is-active="!!processNameModel.length"
          @click.left="focusOnTextField(processNameTextFieldRef)"
        >
          <template #label>Process Name</template>
          <template #content>
            <BaseTextField
              v-model.trim="processNameModel"
              placeholder="node"
              ref="processNameTextFieldRef"
            />
          </template>
        </BaseLabeledBox>
        <BaseLabeledBox
          :background-color="boxBackgroundColor"
          :color="boxColor"
          :active-color="boxActiveColor"
          :is-active="!!processPathModel.length"
          @click.left="focusOnTextField(processPathTextFieldRef)"
        >
          <template #label>Process Path</template>
          <template #content>
            <BaseTextField
              ref="processPathTextFieldRef"
              v-model.trim="processPathModel"
              placeholder="/usr/bin/gnome-software"
            />
          </template>
        </BaseLabeledBox>
      </div>
      <BaseButton text="Reset Filtration" @left-clicked="resetFiltration" />
    </div>
    <PortProcessesTable style="flex-grow: 1" :list="computedProcesses" />
  </section>
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
</style>
