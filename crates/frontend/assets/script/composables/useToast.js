import { ref, provide, inject } from 'vue';

export function provideToast() {
  const toasts = ref([]);

  function addToast(title, message, level = 'info', duration = 5000) {
    const toast = {
      title,
      message,
      level: `bg-${level} text-bg-${level}`,
    };
    toasts.value.push(toast);

    setTimeout(() => {
      removeToast(toasts.value.indexOf(toast));
    }, duration);
  }

  function removeToast(index) {
    toasts.value.splice(index, 1);
  }

  provide('toast', {
    toasts,
    addToast,
    removeToast,
  });
}

export function useToast() {
  const toast = inject('toast');

  if (!toast) {
    throw new Error('No toast provided!');
  }

  return toast;
}
