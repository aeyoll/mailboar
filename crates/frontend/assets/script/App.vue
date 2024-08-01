<template>
  <div class="wrapper">
    <header class="navbar navbar-expand-md d-print-none">
      <div class="container-xl">
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar-menu">
          <span class="navbar-toggler-icon" />
        </button>
        <h1 class="navbar-brand navbar-brand-autodark d-none-navbar-horizontal pe-0 pe-md-3">
          <router-link :to="{ name: 'index' }" class="text-decoration-none d-inline-flex align-items-center gap-2">
            <img src="../images/logo.png" alt="Mailboar" width="40" height="30" />
            Mailboar
          </router-link>
        </h1>
        <div class="navbar-nav flex-row order-md-last align-items-center">
          <span class="navbar-brand-version">v{{ version }}</span>
        </div>
      </div>
    </header>
    <div class="page-wrapper">
      <div class="container-xxl">
        <v-app-header :messages="sortedMessages" />
        <div v-if="sortedMessages.length > 0" class="row">
          <div class="col-sm-4">
            <v-message-list :messages="sortedMessages" />
          </div>
          <div class="col-sm-8">
            <router-view />
          </div>
        </div>
        <div v-else>
          <router-view />
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions, mapGetters, mapState } from 'vuex';
export default {
  name: 'App',
  computed: {
    ...mapState(['apiUrl', 'messages', 'version']),
    ...mapGetters(['sortedMessages']),
  },
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
