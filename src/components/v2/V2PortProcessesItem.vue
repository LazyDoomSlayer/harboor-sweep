<template>
  <div
    :id="elementId"
    :style="computedStyleObject"
    class="process-item"
    @mouseenter="updateHoverStatus(true)"
    @mouseleave="updateHoverStatus(false)"
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
      {{ props.process.is_listener ? 'Hover' : 'Estabilished' }}
    </div>
    <!--    <div class="process-item__actions">-->
    <!--&lt;!&ndash;      <BaseButton&ndash;&gt;-->
    <!--&lt;!&ndash;        :background-color="buttonBackgroundColor"&ndash;&gt;-->
    <!--&lt;!&ndash;        text="Kill"&ndash;&gt;-->
    <!--&lt;!&ndash;        @left-clicked="killProcess"&ndash;&gt;-->
    <!--&lt;!&ndash;      />&ndash;&gt;-->
    <!--      &lt;!&ndash;      <BaseButton text="Details" @left-clicked="checkPort" />&ndash;&gt;-->
    <!--    </div>-->
  </div>
</template>

<script lang="ts" setup>
import type { TPortProcessItem } from '@/types';
import { computed, type CSSProperties, ref, shallowRef } from 'vue';
import { getCssVariable } from '@/utils/theme-helper';

const props = defineProps<{
  process: TPortProcessItem;
  maxItemHeight: number;
}>();

const elementId = shallowRef<string>(`process-item_${props.process.id}`);

const hoveredBackgroundColor = getCssVariable('process-list-item-bg-focused');

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
@use '@/styles/abstracts/_mixins.scss' as mixins;
@use '@/styles/abstracts/_variables.scss' as variables;

.process-item {
  @include mixins.flex-display;
  @include mixins.transition-all('fast');

  flex-grow: 1;
  align-items: center;

  background: var(--process-list-item-bg);
  color: var(--text-process-list-item);
  font-size: 14px;

  & > div {
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

  //&__actions {
  //  width: 10%;
  //  @include mixins.flex-display;
  //  justify-content: flex-end;
  //  gap: 4px;
  //}
}
</style>
