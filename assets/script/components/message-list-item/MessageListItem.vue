<template>
  <tr class="message-list-item">
    <td>{{ from }}</td>
    <td>{{ to }}</td>
    <td>{{ message.subject }}</td>
    <td><abbr :title="formattedDate">{{ relativeDate }}</abbr></td>
    <td><router-link :to="{ name: 'message', params: { id: message.id }}" class="btn btn-white">View</router-link></td>
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
};
</script>
