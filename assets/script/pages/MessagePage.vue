<template>
  <div class="message">
    <div class="row">
      <div class="col-sm-3">
        <div class="card">
          <div class="card-body">
            <v-message-detail-definition :message="message" />
          </div>
        </div>
      </div>

      <div class="col-sm-9">
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
                <pre v-if="format !== 'html'"><code>{{ messageByFormat }}</code></pre>
                <iframe
                  v-else
                  frameborder="0"
                  :srcdoc="messageByFormat"
                  style="width: 1px; min-width: 100%;"
                  :height="htmlIframeHeight"
                />
              </div>

              <div id="tabs-attachments" class="tab-pane">
                Attachments
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'MessagePage',
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
                      frameHeight: height
                    }, '*');
                  }
                }

                window.onload = () => sendPostMessage();
                window.onresize = () => sendPostMessage();
                <` + '/script>'; // This is untended

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
