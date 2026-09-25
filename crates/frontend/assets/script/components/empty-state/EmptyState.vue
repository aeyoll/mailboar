<template>
  <div class="empty">
    <div class="empty-img">
      <img src="../../../images/logo.png" alt="" height="90" width="120">
    </div>
    <p class="empty-title">
      {{ title }}
    </p>
    <p class="empty-subtitle text-muted">
      {{ subtitle }}
    </p>
  </div>
</template>

<script>
export default {
  name: 'EmptyState',
  props: {
    title: {
      type: String,
      default: '',
    },
    subtitle: {
      type: String,
      default: '',
    },
  },
};
</script>

<style scoped lang="scss">
.empty-title {
  font-size: 1.125rem;
}
</style>
