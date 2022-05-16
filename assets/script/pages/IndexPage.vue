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
            <a href="#" class="btn btn-danger" @click.prevent="deleteAllMessages()">
              Delete all messages
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="page-body">
      <v-message-list v-if="messages.length > 0" :messages="messages" />
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
import { mapState } from 'vuex';
export default {
  name: 'IndexPage',
  data: function () {
    return {
      messages: [],
      socket: null,
    };
  },
  computed: mapState({
    apiUrl: state => state.apiUrl,
  }),
  mounted: function () {
    this.getMessages();

    this.socket = new WebSocket('ws://127.0.0.1:3012/');

    this.socket.addEventListener('open', () => {
      console.log('Connected to WebSocket server');
    });

    this.socket.addEventListener('message',  (event) => {
      console.log('Message received:', event.data);
      this.getMessages();
    });
  },
  unmounted() {
    this.socket.close();
  },
  methods: {
    getMessages() {
      this
        .axios
        .get(`${this.apiUrl}/messages`)
        .then((response) => {
          const messages = response.data;
          this.messages = messages.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
        });
    },
    deleteAllMessages() {
      this
        .axios
        .delete(`${this.apiUrl}/messages`)
        .then(() => {
          this.messages = [];
        });
    },
  },
};
</script>
