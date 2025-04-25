<script lang="ts" setup>
import { RouterView } from 'vue-router';
import { onMounted, onUnmounted } from 'vue';
import { useApplicationStore } from '@/store/application.store.ts';

const applicationStore = useApplicationStore();

function handleFocus() {
  applicationStore.isApplicationFocused = true;
}

function handleBlur() {
  applicationStore.isApplicationFocused = false;
}

onMounted(() => {
  window.addEventListener('focus', handleFocus);
  window.addEventListener('blur', handleBlur);
});

onUnmounted(() => {
  window.removeEventListener('focus', handleFocus);
  window.removeEventListener('blur', handleBlur);
});
</script>

<template>
  <RouterView />
</template>

<style lang="scss" scoped></style>
