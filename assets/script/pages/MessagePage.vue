<template>
  <div class="message">
    <div class="row">
      <div class="col-sm-3">
        <div class="card">
          <div class="card-body">
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
          </div>
        </div>
      </div>

      <div class="col-sm-9">
        <div class="card" v-if="message">
          <ul class="nav nav-tabs" data-bs-toggle="tabs">
            <li class="nav-item" v-for="(format, index) in message.formats" :key="format">
              <a :href="'#tabs-' + format" class="nav-link" :class="{ 'active': index === 0 }" data-bs-toggle="tab">{{ format }}</a>
            </li>
          </ul>

          <div class="card-body">
            <div class="tab-content">
              <div class="tab-pane" :id="'tabs-' + format"  v-for="(messageByFormat, format, index) in messageByFormats" :key="format" :class="{ 'active': index === 0 }">
                <pre v-if="format !== 'html'"><code>{{ messageByFormat }}</code></pre>
                <iframe v-else frameborder="0" :srcdoc="messageByFormat" style="width: 100%"></iframe>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { emailMixin } from '../mixins/email';

export default {
  name: 'message-page',
  data: function () {
    return {
      message: null,
      messageByFormats: {},
    };
  },
  methods: {
    getMessage(messageId) {
      return this
        .axios
        .get(`http://127.0.0.1:1080/messages/${messageId}`);
    },
    getFormat(messageId, format) {
      return this
        .axios
        .get(`http://127.0.0.1:1080/messages/${messageId}/${format}`);
    },
    async init() {
      const messageId = this.$route.params.id;

      await this
        .getMessage(messageId)
        .then(response => {
          const message = response.data;
          this.message = message;
        });

      await this.message.formats.forEach(format => {
        this
          .getFormat(messageId, format)
          .then(response => {
            this.$set(this.messageByFormats, format, response.data);
          });
      });
    }
  },
  mixins: [emailMixin],
  mounted: function () {
    this.init();
  }
};
</script>
