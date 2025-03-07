<template>
  <div
    @mouseenter="updateHoverStatus(true)"
    @mouseleave="updateHoverStatus(false)"
    :id="elementId"
    :style="computedStyleObject"
    class="process-item"
  >
    <div class="process-item__pid text-clamp">
      {{ props.process.pid }}
    </div>
    <div class="process-item__port text-clamp">{{ props.process.port }}</div>
    <div class="process-item__process-name text-clamp">
      {{ props.process.process_name }}
    </div>
    <div class="process-item__process-path text-clamp">
      {{ props.process.process_path }}
    </div>
    <div class="process-item__state text-clamp">
      {{ props.process.is_listener ? 'Listen' : 'Estabilished' }}
    </div>
    <div class="process-item__actions">
      <BaseButton
        text="Kill"
        @left-clicked="killProcess"
        :background-color="buttonBackgroundColor"
      />
      <BaseButton text="Details" @left-clicked="checkPort" />
    </div>
  </div>
</template>

<script setup lang="ts">
import BaseButton from '@/components/base/BaseButton.vue';

import type { TPortProcessItem } from '@/types';
import { computed, shallowRef, type CSSProperties, ref, toRaw } from 'vue';

import { storeToRefs } from 'pinia';
import { useDialogsStore } from '@/store/dialogs.store';

const props = defineProps<{
  process: TPortProcessItem;
  maxItemHeight: number;
}>();

const elementId = shallowRef<string>(`process-item_${props.process.id}`);

const hoveredBackgroundColor = getCssVariable('main-background-color-selected');
const buttonBackgroundColor = getCssVariable('negative-color');
const computedStyleObject = computed<CSSProperties>(() => {
  return {
    height: `${props.maxItemHeight}px`,
    backgroundColor: isHoveredOn.value ? hoveredBackgroundColor : 'transparent',
  };
});

const isHoveredOn = ref<boolean>(false);
function updateHoverStatus(hovered: boolean): void {
  isHoveredOn.value = hovered === true;
}

function killProcess(): void {
  confirmKillingDialog.value = {
    opened: true,
    process: toRaw(props.process),
  };
}

const dialogStore = useDialogsStore();
const { confirmKillingDialog } = storeToRefs(dialogStore);
import { invoke } from '@tauri-apps/api/core';
import { getCssVariable } from '@/utils/theme-helper';

async function checkPort(): Promise<void> {
  const { port, pid } = props.process;

  try {
    console.log('before invoke', port, pid);
    const response = await invoke('get_processes_using_port', {
      port,
      itemPid: pid,
    });
    console.log('response', response);
  } catch (error) {
    console.error(error);
  }
}
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;
@use '@/styles/abstracts/_variables.scss' as variables;

.process-item {
  @include mixins.flex-display;
  @include mixins.transition-all('fast');

  flex-grow: 1;
  align-items: center;

  border-bottom: 1px solid variables.get-color('branch-mode-background-color');
  color: variables.get-color('base-label-border-passive-color');

  > div {
    padding: 0px 6px;
  }
  &__pid {
    width: 10%;
  }
  &__port {
    width: 10%;
  }
  &__process-name {
    width: 15%;
  }
  &__process-path {
    width: 40%;
  }
  &__state {
    width: 15%;
  }
  &__actions {
    width: 10%;
    @include mixins.flex-display;
    gap: 4px;
  }
}
</style>
