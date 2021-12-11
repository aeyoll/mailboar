<template>
  <tr class="message-list-item">
    <td>{{ from }}</td>
    <td>{{ to }}</td>
    <td>{{ message.subject }}</td>
    <td>{{ message.created_at }}</td>
    <td><router-link :to="{ name: 'message', params: { id: message.id }}" class="btn btn-white">View</router-link></td>
  </tr>
</template>

<script>
import { parseEmail } from '../../helpers/email';

export default {
  name: 'message-list-item',
  props: {
    message: Object
  },
  computed: {
    from: function () {
      return parseEmail(this.message.sender);
    },
    to: function() {
      return this.message.recipients.map(recipient => parseEmail(recipient)).join(', ');
    }
  }
};
</script>
