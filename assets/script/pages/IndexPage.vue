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
              Delete all messsages
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
    };
  },
  computed: mapState({
    apiAddress: state => state.apiAddress,
  }),
  mounted: function () {
    this
      .axios
      .get(`${this.apiAddress}/messages`)
      .then((response) => {
        const messages = response.data;
        this.messages = messages.sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
      });
  },
  methods: {
    deleteAllMessages() {
      this
        .axios
        .delete(`${this.apiAddress}/messages`)
        .then(() => {
          this.messages = [];
        });
    },
  },
};
</script>
