<template>
  <div class="toast-container position-fixed bottom-0 end-0 p-3">
    <div
      v-for="(toast, index) in toasts"
      :key="index"
      class="toast show"
      :class="toast.level"
      role="alert"
      aria-live="assertive"
      aria-atomic="true"
    >
      <div class="toast-header">
        <strong class="me-auto">{{ toast.title }}</strong>
        <button type="button" class="btn-close" aria-label="Close" @click="removeToast(index)" />
      </div>
      <div v-if="toast.message" class="toast-body">
        {{ toast.message }}
      </div>
    </div>
  </div>
</template>

<script>
import { inject } from 'vue';

export default {
  name: 'ToastNotifications',
  setup() {
    const { toasts, removeToast } = inject('toast');

    return {
      toasts,
      removeToast,
    };
  },
};
</script>

<style scoped lang="scss">
.toast-container {
  z-index: 1050;
}
</style>
