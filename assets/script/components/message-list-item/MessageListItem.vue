<template>
  <tr class="message-list-item">
    <td>{{ message.sender }}</td>
    <td>{{ message.subject }}</td>
    <td>{{ message.created_at }}</td>
    <td><router-link :to="{ name: 'message', params: { id: message.id }}" class="btn btn-white">View</router-link></td>
  </tr>
</template>

<script>
export default {
  name: 'message-list-item',
  props: {
    message: Object
  },
};
</script>
