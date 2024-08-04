<template>
  <div>
    <div v-if="message" class="card mb-3">
      <v-message-detail-definition :message="message" />
    </div>

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

        <div class="col-auto ms-auto">
          <div class="btn-list">
            <button class="btn" @click.prevent="showSendModal = true">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-send"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 14l11 -11" /><path d="M21 3l-6.5 18a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5" /></svg>
              Send
            </button>
            <button class="btn" @click.prevent="deleteMessage()">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-trash"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 7l16 0" /><path d="M10 11l0 6" /><path d="M14 11l0 6" /><path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" /><path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" /></svg>
              Delete
            </button>
          </div>
        </div>
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

      <v-send-message-modal
        v-model:opened="showSendModal"
        @send-message="sendMessage($event)"
      />
    </div>
    <div v-else>
      <v-empty-state title="Message not found" subtitle="The message you are looking for does not exist." />
    </div>
  </div>
</template>

<script>
import { emailMixin } from '../mixins/email';
import { mapState } from 'vuex';
import * as PostalMime from 'postal-mime';
import { useToast } from '../composables/useToast';

export default {
  name: 'MessagePage',

  mixins: [emailMixin],

  setup() {
    const { addToast } = useToast();
    return { addToast };
  },

  data: function () {
    return {
      loading: false,
      activeFormat: 'source',
      message: null,
      parsedMessage: null,
      messageByFormats: {},
      inlineContent: {},
      showSendModal: false,
    };
  },

  computed: mapState({
    ...mapState(['apiUrl']),
  }),

  created() {
    // watch the params of the route to fetch the data again
    this.$watch(
      () => this.$route.params.id,
      this.fetchData,
      // fetch the data when the view is created and the data is
      // already being observed
      { immediate: true }
    );
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
    async sendMessage({ to }) {
      await this
        .axios
        .post(`${this.apiUrl}/messages/${this.message.id}/send`, { to })
        .then(() => {
          this.addToast('Message sent', 'The message has been sent successfully to ' + to, 'success');
          this.showSendModal = false;
        })
        .catch((error) => {
          console.error(error);
          this.addToast('Failed to send the message', 'Please try again later', 'danger');
        });
    },
    openSendModal() {
      this.showSendModal = true;
    },
    async fetchData() {
      // Reset the state
      this.message = null;
      this.parsedMessage = null;
      this.messageByFormats = {};
      this.loading = true;
      this.inlineContent = {};

      // Local state
      const messageId = this.$route.params.id;
      const messageByFormats = {};

      // Fetch the message
      try {
        const { data } = await this.getMessage(messageId);
        const message = data;
        message.formats.sort((a) => a === 'html' ? -1 : 1); // Put the HTML at the top
        this.message = message;

        await Promise.all(this.message.formats.map(async (format) => {
          const { data } = await this.getFormat(messageId, format);
          let content = data;
          messageByFormats[format] = content;

          if (format === 'html') {
            this.activeFormat = 'html';
          } else if (format === 'source') {
            // Parse the email
            const parser = new PostalMime.default();
            parser.parse(content).then(res => {
              this.parsedMessage = res;
            });
          }
        }));

        this.messageByFormats = {};
        this.messageByFormats = {...messageByFormats};
      } catch (error) {
        console.error(error);
      } finally {
        this.loading = false;
      }
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
