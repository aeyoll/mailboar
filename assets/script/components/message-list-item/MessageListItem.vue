<template>
  <tr class="message-list-item" @click.prevent="goToMessage()">
    <td class="message-list-item-from">{{ from }}</td>
    <td class="message-list-item-to">{{ to }}</td>
    <td class="message-list-item-subject">{{ subject }}</td>
    <td><abbr class="message-list-item-date" :title="formattedDate">{{ relativeDate }}</abbr></td>
  </tr>
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
    subject: function () {
      const length = 70;
      if (this.message.subject.length > length) {
        return this.message.subject.substring(0, length) + '...';
      } else {
        return this.message.subject;
      }
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
    cursor: pointer;
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
