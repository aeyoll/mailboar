<template>
  <div v-if="message" class="message-page">
    <div class="page-header">
      <div class="row align-items-center">
        <div class="col">
          <div class="page-pretitle">
            {{ relativeDate }}
          </div>
          <h1 class="page-title">
            {{ message.subject }}
          </h1>
        </div>
        <div class="col-auto ms-auto">
          <div class="btn-list">
            <a href="#" class="btn btn-danger" @click.prevent="deleteMessage()">
              Delete
            </a>
          </div>
        </div>
      </div>
    </div>

    <div class="page-body">
      <div class="row">
        <div class="col-sm-4">
          <div class="card">
            <v-message-detail-definition :message="message" />
          </div>
        </div>

        <div class="col-sm-8">
          <div v-if="message" class="card">
            <div class="card-header">
              <ul class="nav nav-pills card-header-pills" data-bs-toggle="tabs">
                <li v-for="(format) in message.formats" :key="format" class="nav-item">
                  <a :href="'#tabs-' + format" class="nav-link" :class="{ 'active': activeFormat === format }" data-bs-toggle="tab">
                    {{ format }}
                  </a>
                </li>

                <li v-if="message.attachments.length > 0" class="nav-item">
                  <a href="#tabs-attachments" class="nav-link" data-bs-toggle="tab">
                    Attachments
                  </a>
                </li>
              </ul>
            </div>

            <div class="card-body">
              <div class="tab-content">
                <div
                  v-for="(messageByFormat, format) in messageByFormats"
                  :id="'tabs-' + format"
                  :key="format"
                  class="tab-pane"
                  :class="{ 'active': format === activeFormat }"
                >
                  <div v-if="format === 'plain'">
                    {{ messageByFormat }}
                  </div>
                  <div v-if="format === 'source'" class="format-source">
                    {{ messageByFormat }}
                  </div>
                  <v-message-html
                    v-if="format === 'html' && parsedMessage !== null"
                    :parsed-message="parsedMessage"
                  />
                </div>

                <div id="tabs-attachments" class="tab-pane">
                  <v-message-attachments :attachments="message.attachments" />
                </div>
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
import { mapState } from 'vuex';
import * as PostalMime from 'postal-mime';

export default {
  name: 'MessagePage',

  mixins: [emailMixin],

  data: function () {
    return {
      activeFormat: 'source',
      message: null,
      parsedMessage: null,
      messageByFormats: {},
      inlineContent: {},
    };
  },

  computed: mapState({
    apiUrl: state => state.apiUrl,
  }),

  mounted: function () {
    this.init();
  },

  methods: {
    getMessage(messageId) {
      return this
        .axios
        .get(`${this.apiUrl}/messages/${messageId}.json`);
    },
    deleteMessage() {
      this
        .axios
        .delete(`${this.apiUrl}/messages/${this.message.id}`)
        .then(() => {
          this.$router.push({'name': 'index'});
        });
    },
    getFormat(messageId, format) {
      return this
        .axios
        .get(`${this.apiUrl}/messages/${messageId}.${format}`);
    },
    async getSource(messageId) {
      await this
        .getFormat(messageId, 'source')
        .then(response => {
          let content = response.data;
          this.$set(this.messageByFormats, 'source', content);

          // Parse the email
          const parser = new PostalMime.default();
          parser.parse(content).then(res => {
            this.parsedMessage = res;
          });
        });
    },
    async init() {
      const messageId = this.$route.params.id;

      await this
        .getMessage(messageId)
        .then(response => {
          const message = response.data;
          message.formats.sort((a) => a === 'html' ? -1 : 1);
          this.message = message;
        });

      await this.getSource(messageId);

      const formats = this
        .message
        .formats
        .filter(format => ['source'].includes(format) === false);

      await formats.forEach(format => {
        this
          .getFormat(messageId, format)
          .then(response => {
            let content = response.data;
            this.$set(this.messageByFormats, format, content);

            if (format === 'html') {
              this.activeFormat = 'html';
            }
          });
      });
    },
  },
};
</script>

<style scoped lang="scss">
@import '~@tabler/core/src/scss/_config.scss';

.nav-link {
  text-transform: capitalize;
}

.format-source {
  font-family: $font-family-monospace;
  white-space: pre-line;
}
</style>
