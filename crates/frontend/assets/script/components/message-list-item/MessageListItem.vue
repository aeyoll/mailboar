<template>
  <div class="message-list-item" :class="{ 'active': active }" @click.prevent="goToMessage()">
    <div class="d-flex">
      <div class="flex-fill">
        <div class="font-weight-medium">{{ from }}</div>
        <div class="text-muted">{{ to }}</div>
      </div>
      <div class="text-muted ms-auto flex-shrink-0">
        <abbr :title="formattedDate">{{ relativeDate }}</abbr>
      </div>
    </div>
    <div class="text-truncate mt-1">{{ subject }}</div>
  </div>
</template>

<script>
import { emailMixin } from '../../mixins/email';

export default {
  name: 'MessageListItem',
  mixins: [emailMixin],
  props: {
    message: {
      type: Object,
      default() {
        return {};
      },
    },
  },
  computed: {
    active() {
      return 'id' in this.$route.params && parseInt(this.$route.params.id, 10) === this.message.id;
    },
  },
  methods: {
    goToMessage() {
      this.$router.push({ name: 'message', params: { id: this.message.id }});
    },
  },
};
</script>

<style scoped lang="scss">
  .message-list-item {
    background-color: #f8f9fa;
    border-bottom: 1px solid #dee2e6;
    cursor: pointer;
    padding: 1rem;

    // Dark mode
    [data-bs-theme="dark"] & {
      background-color: var(--tblr-bg-surface);
      border-bottom: 1pixx solid var(--tblr-bg-surface-tertiary);
    }

    &:first-of-type {
      border-top-left-radius: var(--tblr-border-radius);
      border-top-right-radius: var(--tblr-border-radius);
    }

    &.active {
      background-color: #fff;

      // Dark mode
      [data-bs-theme="dark"] &{
        background-color: var(--tblr-bg-surface-tertiary);
      }
    }
  }

  .message-list-item-from,
  .message-list-item-to,
  .message-list-item-subject {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .message-list-item-from,
  .message-list-item-to {
    max-width: 200px;
  }

  .message-list-item-date {
    white-space: nowrap;
  }
</style>
