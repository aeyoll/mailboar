<template>
  <div class="card message-list">
    <div class="message-list-wrapper mb-scroll">
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
.message-list {
  overflow: hidden;
}

.message-list-wrapper {
  max-height: var(--mb-list-height);
  overflow-x: hidden;
  overflow-y: auto;
}

// The list only pins once it sits beside the message rather than above it.
@media (min-width: 576px) {
  .message-list {
    position: sticky;
    top: var(--mb-stick-top);
  }
}
</style>
