<script setup lang="ts">
import { ref } from 'vue';

const props = defineProps<{
  placeholder?: string;
  isDisabled?: boolean;
}>();

const model = defineModel();
const textField = ref<HTMLInputElement | null>(null);

function focusField(): void {
  if (!textField.value) {
    throw new Error('Could not find element to focus');
  }

  try {
    textField.value.focus();
  } catch (error) {
    console.error(error);
  }
}

defineExpose({
  focusField,
});
</script>

<template>
  <div class="text-field">
    <input
      ref="textField"
      :class="{ 'disabled-button': props.isDisabled }"
      type="text"
      class="text-field_input"
      v-model="model"
      :placeholder="props.placeholder"
    />
  </div>
</template>

<style lang="scss" scoped>
@use '@/styles/abstracts/_mixins.scss' as mixins;
@use '@/styles/abstracts/_variables.scss' as variables;

.text-field {
  width: 100%;
  height: 100%;
  @include mixins.flex-display;

  &_input {
    width: 100%;
    height: 100%;

    border: none;
    outline: none;

    color: variables.get-color('base-label-border-passive-color');

    background-color: transparent;
  }
}

.disabled-button {
  @include mixins.disable-user-selection;
}
</style>
