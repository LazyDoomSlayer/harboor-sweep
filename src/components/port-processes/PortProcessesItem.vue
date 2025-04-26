<template>
  <div
    :id="elementId"
    :style="computedStyleObject"
    class="process-item"
    @click.left="focusOnProcess"
  >
    <div class="process-item__pid hide-text-overflow-line-1">
      {{ props.process.pid }}
    </div>
    <div class="process-item__port hide-text-overflow-line-1">
      {{ props.process.port }}
    </div>
    <div class="process-item__process-name hide-text-overflow-line-1">
      {{ props.process.process_name }}
    </div>
    <div class="process-item__process-path hide-text-overflow-line-1">
      {{ props.process.process_path }}
    </div>
    <div class="process-item__state hide-text-overflow-line-1">
      {{ props.process.is_listener ? 'Hosting' : 'Using' }}
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { TPortProcessItem } from '@/types';
import { computed, type CSSProperties, shallowRef } from 'vue';
import { getCssVariable } from '@/utils/theme-helper.ts';
import { usePortProcessesStore } from '@/store/port-processes.store.ts';

const props = defineProps<{
  process: TPortProcessItem;
  maxItemHeight: number;
}>();

const portProcessesStore = usePortProcessesStore();

const elementId = shallowRef<string>(`process-item_${props.process.id}`);

const hoveredBackgroundColor = getCssVariable('process-list-item-bg-focused');

const computedStyleObject = computed<CSSProperties>(() => {
  return {
    height: `${props.maxItemHeight}px`,
    backgroundColor:
      portProcessesStore.processFocused?.id === props.process?.id
        ? hoveredBackgroundColor
        : 'transparent',
  };
});

// const isHoveredOn = ref<boolean>(false);

// function updateHoverStatus(hovered: boolean): void {
//   isHoveredOn.value = hovered === true;
// }

function focusOnProcess() {
  portProcessesStore.processFocused = props.process;
}

// function killProcess(): void {
//   confirmKillingDialog.value = {
//     opened: true,
//     process: toRaw(props.process),
//   };
// }

// const dialogStore = useDialogsStore();
// const { confirmKillingDialog } = storeToRefs(dialogStore);

// async function checkPort(): Promise<void> {
//   const { port, pid } = props.process;
//
//   try {
//     console.log('before invoke', port, pid);
//     const response = await invoke('get_processes_using_port', {
//       port,
//       itemPid: pid,
//     });
//     console.log('response', response);
//   } catch (error) {
//     console.error(error);
//   }
// }
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/mixins' as mixins;
@use '@/styles/abstracts/variables' as variables;

.process-item {
  @include mixins.flex-display;
  @include mixins.transition-all('fast');

  flex-grow: 1;
  align-items: center;

  background: var(--process-list-item-bg);
  color: var(--text-process-list-item);
  font-size: 12px;

  &__pid,
  &__port,
  &__process-name,
  &__process-path,
  &__state {
    padding: 0 6px;
    line-height: 20px;
    max-height: 20px;
  }

  &__pid {
    width: 10%;
  }

  &__port {
    width: 10%;
  }

  &__process-name {
    width: 25%;
  }

  &__process-path {
    width: 40%;
  }

  &__state {
    width: 15%;
  }
}
</style>
