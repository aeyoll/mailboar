<template>
  <div class="index-page">
    <div class="page-header">
      <div class="row align-items-center">
        <div class="col">
          <div class="page-title">
            You have {{ messages.length }} message<template v-if="messages.length > 1">s</template>
          </div>
        </div>
        <div v-if="messages.length > 0" class="col-auto ms-auto">
          <div class="btn-list">
            <a href="#" class="btn btn-danger" @click.prevent="deleteMessages()">
              Delete all messages
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="page-body">
      <v-message-list v-if="sortedMessages.length > 0" :messages="sortedMessages" />
      <div v-else class="container-xl d-flex flex-column justify-content-center">
        <div class="empty">
          <div class="empty-img">
            <img src="../../images/undraw_empty_street_sfxm.svg" alt="" height="128">
          </div>
          <p class="empty-title">
            No mail to show (yet!)
          </p>
          <p class="empty-subtitle text-muted">
            Send one and refresh this page.
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions, mapState } from 'vuex';
export default {
  name: 'IndexPage',
  computed: mapState({
    ...mapState(['apiUrl', 'messages']),
    sortedMessages: function () {
      return this.messages.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
    },
  }),
  mounted: function () {
    this.fetchMessages();
    this.subscribeToSSE();
  },
  beforeUnmount: function () {
    this.unsubscribeFromSSE();
  },
  methods: {
    ...mapActions(['fetchMessages', 'addMessage', 'deleteMessages']),
    subscribeToSSE() {
      this.eventSource = new EventSource(`${this.apiUrl}/events`);
      this.eventSource.onmessage = (event) => {
        const message = JSON.parse(event.data);
        this.addMessage(message);
      };
      this.eventSource.onerror = (error) => {
        console.error('SSE Error:', error);
        this.unsubscribeFromSSE();
        // Attempt to reconnect after 5 seconds
        setTimeout(() => this.subscribeToSSE(), 5000);
      };
    },
    unsubscribeFromSSE() {
      if (this.eventSource) {
        this.eventSource.close();
        this.eventSource = null;
      }
    },
  },
};
</script>
