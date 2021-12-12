<template>
  <div class="card message-list">
    <div class="table-responsive">
      <table class="table table-vcenter table-striped card-table">
        <thead>
          <tr>
            <th>From</th>
            <th>To</th>
            <th>Subject</th>
            <th>Date</th>
          </tr>
        </thead>
        <tbody>
          <v-message-list-item
            v-for="message in messages"
            :key="message.id"
            :message="message"
          />
        </tbody>
      </table>
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
