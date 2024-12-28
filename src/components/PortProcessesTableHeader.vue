<template>
  <div class="process-table-header">
    <div class="process-table-header__pid">PID</div>
    <div class="process-table-header__port">Port</div>
    <div class="process-table-header__process-name">Process Name</div>
    <div class="process-table-header__process-path">Process Path</div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';

onMounted(() => {
  console.log('hello from ehader');
});
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;
.process-table-header {
  @include mixins.flex-display;

  flex-grow: 1;
  border: 2px solid #3e3e3e;
  padding: 4px;
  border-radius: 4px;
  > div {
    height: 32px;
    padding: 0px 6px;

    @include mixins.flex-display;

    align-items: center;
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
