<template>
  <tr class="message-list-item" @click.prevent="goToMessage()">
    <td>{{ from }}</td>
    <td>{{ to }}</td>
    <td>{{ message.subject }}</td>
    <td><abbr :title="formattedDate">{{ relativeDate }}</abbr></td>
  </tr>
</template>

<script>
import { emailMixin } from '../../mixins/email';

export default {
  name: 'message-list-item',
  props: {
    message: Object,
  },
  mixins: [emailMixin],
  methods: {
    goToMessage() {
      this.$router.push({ name: 'message', params: { id: this.message.id }});
    }
  }
};
</script>

<style scoped lang="scss">
  .message-list-item {
    cursor: pointer;
  }
</style>
