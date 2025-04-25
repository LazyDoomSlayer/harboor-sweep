<template>
  <div class="process-table-header">
    <div class="process-table-header__pid" @click.left="changeSorting('pid')">
      PID
      <span v-if="sorting.key === 'pid'">
        {{ sorting.direction === ESorting.ASCENDING ? '↑' : '↓' }}
      </span>
    </div>
    <div class="process-table-header__port" @click.left="changeSorting('port')">
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
  </div>
</template>

<script lang="ts" setup>
import { storeToRefs } from 'pinia';
import { usePortProcessesStore } from '@/store/port-processes.store';

import { ESorting, type TPortProcessItem } from '@/types';

// const boxBackgroundColor = getCssVariable('main-background-color');
// const boxColor = getCssVariable('base-label-border-passive-color');
// const boxActiveColor = getCssVariable('dialog-active-color');

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
  color: var(--text-process-list-header);

  & > div {
    height: 32px;
    padding: 0 6px;

    @include mixins.flex-display;
    border-bottom: 1px solid var(--main-divider-bg);

    align-items: center;
    font-size: 12px;

    &:hover {
      color: var(--text-process-list-header-hovered);
    }

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
    width: 25%;
    cursor: pointer;
  }

  &__process-path {
    width: 40%;
  }

  &__state {
    width: 15%;
  }
}
</style>
