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
import { parseEmail } from '../../helpers/email';

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime)

export default {
  name: 'message-list-item',
  props: {
    message: Object,
  },
  computed: {
    from: function () {
      return parseEmail(this.message.sender);
    },
    to: function() {
      return this.message.recipients.map(recipient => parseEmail(recipient)).join(', ');
    },
    formattedDate: function () {
      return dayjs(this.message.created_at).format('DD MMMM YYYY, hh:mm:ss');
    },
    relativeDate: function () {
      return dayjs().to(dayjs(this.message.created_at));
    },
  }
};
</script>
