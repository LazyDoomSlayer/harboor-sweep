<template>
  <div
    :style="styleObject"
    class="toast-item"
    @mouseenter="isHovered = true"
    @mouseleave="isHovered = false"
  >
    <span
      v-if="props.data.prependIcon"
      class="toast-item__icon toast-item__icon--prepend"
    >
      <span class="material-symbols-outlined">{{
        props.data.prependIcon
      }}</span>
    </span>

    <div class="toast-item__content">
      <div class="toast-item__title">{{ props.data.title }}</div>
      <div v-if="props.data.description" class="toast-item__description">
        {{ props.data.description }}
      </div>
    </div>

    <span
      v-if="props.data.appendIcon"
      class="toast-item__icon toast-item__icon--append"
    >
      <span class="material-symbols-outlined">{{ props.data.appendIcon }}</span>
    </span>

    <span
      class="material-symbols-outlined toast-item__close"
      @click="$emit('removeToast', props.data.id)"
    >
      {{ props.data.closeIcon ?? 'close' }}
    </span>
  </div>
</template>

<script lang="ts" setup>
import type { IToastNotification } from '@/types/store/notifications.types';
import { type CSSProperties, onMounted, ref } from 'vue';

interface IToastNotificationItemProps {
  data: IToastNotification;
  index: number;
}

const props = defineProps<IToastNotificationItemProps>();
const emit = defineEmits<{
  (e: 'removeToast', toast_id: IToastNotification['id']): void;
}>();

const styleObject: CSSProperties = {
  color: props.data.textColor || '#ffffff',
  backgroundColor: props.data.backgroundColor || '#36363a',
};

const isHovered = ref<boolean>(false);
const duration: number = props.data.duration ?? 2_000;

let timeout: number | null = null;

onMounted(() => {
  removeNotification();
});

function removeNotification(): void {
  if (timeout !== null) {
    clearTimeout(timeout);
  }

  timeout = setTimeout(() => {
    if (isHovered.value) {
      removeNotification();
      return;
    }

    emit('removeToast', props.data.id);
  }, duration);
}
</script>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;

.toast-item {
  @include mixins.flex-display;
  @include mixins.justify-content-space-between;
  @include mixins.align-content-center;
  @include mixins.transition-all('medium');

  padding: 6px 12px;
  border-radius: 6px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.5);
  border: 2px solid var(--main-divider-bg);
  font-size: 12px;

  min-width: 220px;
  max-width: 260px;

  overflow: hidden;

  &:hover {
    transform: scale(1.05);
  }

  &__content {
    flex: 1;
    margin: 0 10px;
  }

  &__title {
    font-weight: inherit;
  }

  &__description {
    font-size: 12px;
    color: var(--text-active);
  }

  &__icon {
    @include mixins.flex-display;
    @include mixins.justify-content-center;
    @include mixins.align-content-center;

    font-size: 20px;

    &--prepend {
      margin-right: 10px;
    }

    &--append {
      margin-left: 10px;
    }
  }

  &__close {
    cursor: pointer;
    margin-left: 10px;
    color: var(--text-active);

    font-size: 16px;
  }
}
</style>
