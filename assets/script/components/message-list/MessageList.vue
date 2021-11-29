<template>
  <div class="card message-list">
    <div class="table-responsive">
      <table class="table table-vcenter card-table">
        <thead>
          <tr>
            <th>From</th>
            <th>Subject</th>
            <th>Date</th>
            <th class="w-1"></th>
          </tr>
        </thead>
        <tbody>
          <v-message-list-item v-for="message in messages" :key="message.id" :message="message">
          </v-message-list-item>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script>
export default {
  name: 'message-list',
  props: {
    messages: Array
  },
};
</script>
