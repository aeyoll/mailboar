<template>
  <div class="index-page">
    <div class="page-header">
      <div class="page-title">
        You have {{ messages.length }} message<template v-if="messages.length > 1">s</template>
      </div>
      <div v-if="messages.length > 0" class="col-auto ms-auto">
        <div class="btn-list">
          <a href="#" class="btn btn-danger" @click.prevent="deleteAllMessages()">
            Delete all messsages
          </a>
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
export default {
  name: 'IndexPage',
  data: function () {
    return {
      messages: [],
    };
  },
  mounted: function () {
    this
      .axios
      .get('http://127.0.0.1:1080/messages')
      .then((response) => {
        const messages = response.data;
        this.messages = messages.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
      });
  },
  methods: {
    deleteAllMessages() {
      this
        .axios
        .delete('http://127.0.0.1:1080/messages')
        .then(() => {
          this.messages = [];
        });
    },
  },
};
</script>
