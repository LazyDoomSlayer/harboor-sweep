<template>
  <BaseLabeledBox
    class="process-table-header"
    :background-color="boxBackgroundColor"
    :color="boxColor"
    :active-color="boxActiveColor"
    :is-active="false"
  >
    <template #label> Headers </template>
    <template #content>
      <div class="process-table-header__pid" @click.left="changeSorting('pid')">
        PID
        <span v-if="sorting.key === 'pid'">
          {{ sorting.direction === ESorting.ASCENDING ? '↑' : '↓' }}
        </span>
      </div>
      <div
        class="process-table-header__port"
        @click.left="changeSorting('port')"
      >
        Port
        <span v-if="sorting.key === 'port'">
          {{ sorting.direction === ESorting.ASCENDING ? '↑' : '↓' }}
        </span>
      </div>
      <div
        class="process-table-header__process-name"
        @click.left="changeSorting('process_name')"
      >
        Process Name
        <span v-if="sorting.key === 'process_name'">
          {{ sorting.direction === ESorting.ASCENDING ? '↑' : '↓' }}
        </span>
      </div>

      <div class="process-table-header__process-path">Process Path</div>
      <div class="process-table-header__state">State</div>

      <div class="process-table-header__actions">Actions</div>
    </template>
  </BaseLabeledBox>
</template>

<script setup lang="ts">
import BaseLabeledBox from './base/BaseLabeledBox.vue';

import { storeToRefs } from 'pinia';
import { usePortProcessesStore } from '@/store/port-processes.store';

import { ESorting, type TPortProcessItem } from '@/types';
import { getCssVariable } from '@/utils/theme-helper';

const boxBackgroundColor = getCssVariable('main-background-color');
const boxColor = getCssVariable('base-label-border-passive-color');
const boxActiveColor = getCssVariable('dialog-active-color');

const portProcessesStore = usePortProcessesStore();
const { sorting } = storeToRefs(portProcessesStore);

function changeSorting(key: keyof TPortProcessItem): void {
  sorting.value = {
    key,
    direction:
      sorting.value.direction === ESorting.ASCENDING
        ? ESorting.DESCEDNING
        : ESorting.ASCENDING,
  };
}
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;
@use '@/styles/abstracts/_variables.scss' as variables;

.process-table-header {
  @include mixins.flex-display;
  color: variables.get-color('base-label-border-passive-color');

  > div {
    height: 32px;
    padding: 0px 6px;

    @include mixins.flex-display;

    align-items: center;
    > span {
      margin-left: 4px;
    }
  }

  &__pid {
    width: 10%;
    cursor: pointer;
  }
  &__port {
    width: 10%;
    cursor: pointer;
  }
  &__process-name {
    width: 15%;
    cursor: pointer;
  }
  &__process-path {
    width: 40%;
  }
  &__state {
    width: 15%;
  }
  &__actions {
    width: 10%;
  }
}
</style>
