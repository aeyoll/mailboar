<template>
  <div class="message card">
    <div class="card-body">
      <div class="row">
        <div class="col-sm-6">
          <dl class="row">
            <dt class="col-sm-2">From</dt>
            <dd class="col-sm-9">
              <span v-if="message && message.sender">{{ message.sender }}</span>
              <div v-else class="skeleton-line skeleton-line-full"></div>
            </dd>
            <dt class="col-sm-2">Subject</dt>
            <dd class="col-sm-9">
              <span v-if="message && message.subject">{{ message.subject }}</span>
              <div v-else class="skeleton-line skeleton-line-full"></div>
            </dd>
            <dt class="col-sm-2">To</dt>
            <dd class="col-sm-9">
              <span v-if="message && message.recipients">{{ message.recipients.join(', ') }}</span>
              <div v-else class="skeleton-line skeleton-line-full"></div>
            </dd>
          </dl>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'message-page',
  data: function () {
    return {
      message: null
    };
  },
  mounted: function () {
    const messageId = this.$route.params.id;
    this
      .axios
      .get(`http://127.0.0.1:1080/messages/${messageId}`)
      .then((response) => {
        const message = response.data;
        this.message = message;
      });
  }
};
</script>
