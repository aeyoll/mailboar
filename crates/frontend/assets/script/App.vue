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
        <div class="navbar-nav flex-row order-md-last align-items-center gap-2">
          <a href="https://github.com/aeyoll/mailboar" class="btn" target="_blank" rel="noreferrer">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M9 19c-4.3 1.4 -4.3 -2.5 -6 -3m12 5v-3.5c0 -1 .1 -1.4 -.5 -2c2.8 -.3 5.5 -1.4 5.5 -6a4.6 4.6 0 0 0 -1.3 -3.2a4.2 4.2 0 0 0 -.1 -3.2s-1.1 -.3 -3.5 1.3a12.3 12.3 0 0 0 -6.2 0c-2.4 -1.6 -3.5 -1.3 -3.5 -1.3a4.2 4.2 0 0 0 -.1 3.2a4.6 4.6 0 0 0 -1.3 3.2c0 4.6 2.7 5.7 5.5 6c-.6 .6 -.6 1.2 -.5 2v3.5"></path></svg>
            Source code
          </a>

          <button @click="toggleTheme()" class="nav-link px-0 hide-theme-dark" data-bs-toggle="tooltip" data-bs-placement="bottom" aria-label="Enable dark mode" data-bs-original-title="Enable dark mode">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M12 3c.132 0 .263 0 .393 0a7.5 7.5 0 0 0 7.92 12.446a9 9 0 1 1 -8.313 -12.454z"></path></svg>
          </button>

          <button @click="toggleTheme()" class="nav-link px-0 hide-theme-light" data-bs-toggle="tooltip" data-bs-placement="bottom" aria-label="Enable light mode" data-bs-original-title="Enable light mode">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon"><path stroke="none" d="M0 0h24v24H0z" fill="none"></path><path d="M12 12m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0"></path><path d="M3 12h1m8 -9v1m8 8h1m-9 8v1m-6.4 -15.4l.7 .7m12.1 -.7l-.7 .7m0 11.4l.7 .7m-12.1 -.7l-.7 .7"></path></svg>
          </button>

          <span class="navbar-brand-version">
            v{{ version }}
          </span>
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
  data: function() {
    return {
      theme: 'light',
    };
  },
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
    toggleTheme() {
      if (this.theme === 'dark') {
        this.theme = 'light';
      } else {
        this.theme = 'dark';
      }

      document.documentElement.setAttribute('data-theme', this.theme);
      document.documentElement.setAttribute('style', 'color-scheme: ' + this.theme);
      document.getElementsByTagName('body')[0].setAttribute('data-bs-theme', this.theme);
    },
  },
};
</script>
