<template>
  <div class="card message-list">
    <div class="message-list-wrapper">
      <v-message-list-item
        v-for="message in messages"
        :key="message.id"
        :message="message"
      />
    </div>
  </div>
</template>

<script>
export default {
  name: 'MessageList',
  props: {
    messages: {
      type: Array,
      default() {
        return [];
      },
    },
  },
};
</script>

<style scoped lang="scss">
.message-list-wrapper {
  height: 80vh;
  overflow-x: hidden;
  overflow-y: auto;
}
</style>
