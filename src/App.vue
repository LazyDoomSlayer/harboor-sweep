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

function handleContextMenu(event: MouseEvent) {
  event.preventDefault();
}

onMounted(() => {
  window.addEventListener('focus', handleFocus);
  window.addEventListener('blur', handleBlur);
  window.addEventListener('contextmenu', handleContextMenu);
});

onUnmounted(() => {
  window.removeEventListener('focus', handleFocus);
  window.removeEventListener('blur', handleBlur);
  window.removeEventListener('contextmenu', handleContextMenu);
});
</script>

<template>
  <RouterView />
</template>

<style lang="scss" scoped></style>
