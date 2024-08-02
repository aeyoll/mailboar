<template>
  <div>
    <div class="page-body">
      <div v-if="sortedMessages.length == 0" class="container-xl d-flex flex-column justify-content-center">
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
import { mapActions, mapGetters, mapState } from 'vuex';
export default {
  name: 'IndexPage',
  computed: mapState({
    ...mapState(['apiUrl', 'messages']),
    ...mapGetters(['sortedMessages']),
  }),
  mounted() {
    this.fetchMessages();
  },
  methods: {
    ...mapActions(['fetchMessages']),
  },
};
</script>
