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
    <div class="process-table-header__actions">Actions</div>
  </div>
</template>

<script setup lang="ts">
import { usePortProcessesStore } from '@/store/port-processes.store';
import { ESorting, type TPortProcessItem } from '@/types';
import { storeToRefs } from 'pinia';

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

.process-table-header {
  @include mixins.flex-display;

  border: 2px solid #3e3e3e;
  padding: 4px;
  border-radius: 4px;

  > div {
    height: 32px;
    padding: 0px 6px;

    @include mixins.flex-display;

    align-items: center;
    cursor: pointer;
  }

  &__pid {
    width: 10%;
  }
  &__port {
    width: 10%;
  }
  &__process-name {
    width: 30%;
  }
  &__process-path {
    width: 40%;
  }
  &__actions {
    width: 10%;
  }
}
</style>
