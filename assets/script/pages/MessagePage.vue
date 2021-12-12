<template>
  <div class="message">
    <div class="row">
      <div class="col-sm-3">
        <div class="card">
          <div class="card-body">
            <v-message-detail-definition :message="message"></v-message-detail-definition>
          </div>
        </div>
      </div>

      <div class="col-sm-9">
        <div class="card" v-if="message">
          <ul class="nav nav-tabs" data-bs-toggle="tabs">
            <li class="nav-item" v-for="(format, index) in message.formats" :key="format">
              <a :href="'#tabs-' + format" class="nav-link" :class="{ 'active': index === 0 }" data-bs-toggle="tab">{{ format | capitalize }}</a>
            </li>
          </ul>

          <div class="card-body">
            <div class="tab-content">
              <div class="tab-pane" :id="'tabs-' + format"  v-for="(messageByFormat, format, index) in messageByFormats" :key="format" :class="{ 'active': index === 0 }">
                <pre v-if="format !== 'html'"><code>{{ messageByFormat }}</code></pre>
                <iframe
                  v-else
                  frameborder="0"
                  :srcdoc="messageByFormat"
                  style="width: 1px; min-width: 100%;"
                  :height="htmlIframeHeight"></iframe>
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
  name: 'message-page',
  data: function () {
    return {
      message: null,
      messageByFormats: {},
      htmlIframeHeight: 0,
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
    receiveMessage (event) {
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
                <\/script>`;

              content = content.replace('</body>', postMessage + '</body>');
            }

            this.$set(this.messageByFormats, format, content);
          });
      });
    }
  },
  mounted: function () {
    window.addEventListener('message', this.receiveMessage);
    this.init();
  },
  beforeDestroy () {
    window.removeEventListener('message', this.receiveMessage)
  }
};
</script>
