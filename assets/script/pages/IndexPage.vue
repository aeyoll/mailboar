<template>
  <v-message-list v-if="messages.length > 0" :messages="messages"></v-message-list>
  <div v-else class="container-xl d-flex flex-column justify-content-center">
    <div class="empty">
      <div class="empty-img"><img src="../../images/undraw_empty_street_sfxm.svg" alt="" height="128">
      </div>
      <p class="empty-title">No mail to show (yet!)</p>
      <p class="empty-subtitle text-muted">
        Send one and refresh this page.
      </p>
    </div>
  </div>
</template>

<script>
export default {
  name: 'index-page',
  data: function () {
    return {
      messages: []
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
  }
};
</script>
