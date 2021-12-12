<template>
  <dl class="row">
    <dt class="col-sm-4">From</dt>
    <dd class="col-sm-8">
      <span v-if="message && from">{{ from }}</span>
      <div v-else class="skeleton-line skeleton-line-full"></div>
    </dd>
    <dt class="col-sm-4">Subject</dt>
    <dd class="col-sm-8">
      <span v-if="message && message.subject">{{ message.subject }}</span>
      <div v-else class="skeleton-line skeleton-line-full"></div>
    </dd>
    <dt class="col-sm-4">To</dt>
    <dd class="col-sm-8">
      <span v-if="message && to">{{ to }}</span>
      <div v-else class="skeleton-line skeleton-line-full"></div>
    </dd>
  </dl>
</template>

<script>
import { emailMixin } from '../../mixins/email';

export default {
  name: 'message-detail-definition',
  mixins: [emailMixin],
  props: {
    message: Object
  },
};
</script>
