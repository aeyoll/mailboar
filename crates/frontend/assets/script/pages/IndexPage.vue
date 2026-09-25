<template>
  <div>
    <div class="page-body">
      <div v-if="sortedMessages.length == 0" class="container-xl d-flex flex-column justify-content-center">
        <v-empty-state title="No mail caught yet" subtitle="Point your app's SMTP at this server. New messages appear here on their own." />
      </div>
    </div>
  </div>
</template>

<script>
import {mapGetters, mapState } from 'vuex';
export default {
  name: 'IndexPage',
  computed: mapState({
    ...mapState(['apiUrl', 'messages']),
    ...mapGetters(['sortedMessages']),
  }),
};
</script>
