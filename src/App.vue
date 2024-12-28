<script setup lang="ts">
import { onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { RouterView } from 'vue-router';

onMounted(() => {
  greet();
});

async function greet() {
  const toLog = await invoke('greet', { name: 'LazyDoomSlayer' });
  console.log(toLog);
}
</script>

<template>
  <RouterView />
</template>

<style lang="scss" scoped></style>
