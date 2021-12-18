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
            <ul class="nav nav-tabs" data-bs-toggle="tabs">
              <li v-for="(format, index) in message.formats" :key="format" class="nav-item">
                <a :href="'#tabs-' + format" class="nav-link" :class="{ 'active': index === 0 }" data-bs-toggle="tab">
                  {{ format }}
                </a>
              </li>

              <li v-if="message.attachments.length > 0" class="nav-item">
                <a href="#tabs-attachments" class="nav-link" data-bs-toggle="tab">
                  Attachments
                </a>
              </li>
            </ul>

            <div class="card-body">
              <div class="tab-content">
                <div
                  v-for="(messageByFormat, format, index) in messageByFormats"
                  :id="'tabs-' + format"
                  :key="format"
                  class="tab-pane"
                  :class="{ 'active': index === 0 }"
                >
                  <div v-if="format === 'plain'">
                    {{ messageByFormat }}
                  </div>
                  <pre v-if="format === 'source'"><code>{{ messageByFormat }}</code></pre>
                  <iframe
                    v-if="format === 'html'"
                    frameborder="0"
                    :srcdoc="messageByFormat"
                    style="width: 1px; min-width: 100%;"
                    :height="htmlIframeHeight"
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

export default {
  name: 'MessagePage',
  mixins: [emailMixin],
  data: function () {
    return {
      message: null,
      messageByFormats: {},
      htmlIframeHeight: 0,
    };
  },
  mounted: function () {
    window.addEventListener('message', this.receiveMessageFromIframe);
    this.init();
  },
  beforeUnmount () {
    window.removeEventListener('message', this.receiveMessageFromIframe);
  },
  methods: {
    getMessage(messageId) {
      return this
        .axios
        .get(`http://127.0.0.1:1080/messages/${messageId}`);
    },
    deleteMessage() {
      this
        .axios
        .delete(`http://127.0.0.1:1080/messages/${this.message.id}`)
        .then(() => {
          this.$router.push({'name': 'index'});
        });
    },
    getFormat(messageId, format) {
      return this
        .axios
        .get(`http://127.0.0.1:1080/messages/${messageId}/${format}`);
    },
    receiveMessageFromIframe(event) {
      if ('frameHeight' in event.data) {
        this.htmlIframeHeight = event.data.frameHeight;
      }
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
            let content = response.data;

            if (format === 'html') {
              const postMessage = `
                <script>
                const sendPostMessage = () => {
                  if (height !== document.querySelector('body').offsetHeight) {
                    var height = document.querySelector('body').offsetHeight;
                    window.parent.postMessage({
                      frameHeight: height + 50
                    }, '*');
                  }
                }

                window.onload = () => sendPostMessage();
                window.onresize = () => sendPostMessage();
                <` + '/script>'; // This is intended

              if (content.indexOf('</body>') === -1) {
                content = '<body>' + content + '</body>';
              }

              content = content.replace('</body>', postMessage + '</body>');
            }

            this.$set(this.messageByFormats, format, content);
          });
      });
    },
  },
};
</script>

<style scoped lang="scss">
.nav-link {
  text-transform: capitalize;
}
</style>
