<template>
  <div class="empty">
    <div class="empty-img">
      <img src="../../../images/undraw_empty_street_sfxm.svg" alt="" height="128" width="284" class="img-fluid">
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
.empty-img {
  max-height: 128px;
}
</style>
