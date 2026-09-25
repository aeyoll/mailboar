<template>
  <div>
    <div v-if="message" class="card">
      <div class="message-header">
        <div class="message-header-top">
          <h1 class="message-subject">
            {{ subject }}
          </h1>

          <div class="btn-list message-actions">
            <button v-if="configuration.can_forward" class="btn btn-primary" @click.prevent="showSendModal = true">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-send"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 14l11 -11" /><path d="M21 3l-6.5 18a.55 .55 0 0 1 -1 0l-3.5 -7l-7 -3.5a.55 .55 0 0 1 0 -1l18 -6.5" /></svg>
              Forward
            </button>
            <button class="btn btn-ghost-danger" @click.prevent="deleteMessage()">
              <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-trash"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 7l16 0" /><path d="M10 11l0 6" /><path d="M14 11l0 6" /><path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" /><path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" /></svg>
              Delete
            </button>
          </div>
        </div>

        <div class="message-meta">
          <span class="avatar avatar-sm message-meta-avatar">{{ senderInitial }}</span>

          <div class="message-meta-body">
            <dl class="message-route">
              <dt class="message-route-label">
                From
              </dt>
              <dd class="message-route-value">
                {{ from }}
              </dd>
              <dt class="message-route-label">
                To
              </dt>
              <dd class="message-route-value">
                {{ to }}
              </dd>
            </dl>

            <p class="message-stamp mb-numeric text-muted">
              <abbr class="message-stamp-time" :title="formattedDate">{{ relativeDate }}</abbr>
              · {{ humanSize }} · {{ message.type }}
            </p>
          </div>
        </div>
      </div>

      <div class="message-formats">
        <ul class="nav nav-segmented" data-bs-toggle="tabs">
          <li v-for="(format) in message.formats" :key="format" class="nav-item">
            <a :href="'#tabs-' + format" class="nav-link" :class="{ 'active': activeFormat === format }" data-bs-toggle="tab">
              {{ formatLabel(format) }}
            </a>
          </li>

          <li v-if="message.attachments.length > 0" class="nav-item">
            <a href="#tabs-attachments" class="nav-link" data-bs-toggle="tab">
              Attachments
              <span class="badge bg-secondary-lt ms-1">{{ message.attachments.length }}</span>
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
            <pre v-if="format === 'plain'" class="message-plain">{{ messageByFormat }}</pre>
            <pre v-if="format === 'source'" class="message-source mb-scroll">{{ messageByFormat }}</pre>
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

    <div v-else-if="loading" class="card placeholder-glow">
      <div class="message-header">
        <div class="placeholder col-7 mb-3" />
        <div class="placeholder col-4" />
      </div>
    </div>

    <v-empty-state v-else title="Message not found" subtitle="The message you are looking for does not exist." />
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
    ...mapState(['apiUrl', 'configuration']),
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
    formatLabel(format) {
      return { html: 'HTML', plain: 'Plain text', source: 'Source' }[format] ?? format;
    },
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
          this.addToast('Message forwarded', 'A copy is on its way to ' + to, 'success');
          this.showSendModal = false;
        })
        .catch((error) => {
          console.error(error);
          this.addToast('Could not forward the message', 'The relay refused it. Check MAILBOAR_SMTP_DSN and try again.', 'danger');
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
.message-header {
  border-bottom: 1px solid var(--tblr-border-color);
  padding: 1.25rem;
}

.message-header-top {
  align-items: flex-start;
  display: flex;
  gap: 1rem;
  justify-content: space-between;
}

.message-subject {
  font-size: 1.25rem;
  font-weight: var(--tblr-font-weight-bold);
  letter-spacing: -.01em;
  line-height: 1.35;
  margin: 0;
  text-wrap: balance;
}

.message-actions {
  flex: none;
}

.message-meta {
  align-items: flex-start;
  display: flex;
  gap: .75rem;
  margin-top: 1rem;
}

.message-meta-avatar {
  flex: none;
  font-weight: var(--tblr-font-weight-bold);
}

.message-route {
  column-gap: .75rem;
  display: grid;
  grid-template-columns: auto minmax(0, 1fr);
  margin: 0;
  min-width: 0;
  row-gap: .125rem;
}

.message-route-label {
  color: var(--tblr-secondary);
  font-weight: var(--tblr-font-weight-normal);
}

.message-route-value {
  margin: 0;
  overflow-wrap: anywhere;
}

.message-meta-body {
  min-width: 0;
}

.message-stamp {
  font-size: .75rem;
  margin: .25rem 0 0;
}

.message-stamp-time {
  border: 0;
  text-decoration: none;
}

.message-formats {
  border-bottom: 1px solid var(--tblr-border-color);
  padding: .75rem 1.25rem;
}

.message-plain {
  color: var(--tblr-body-color);
  font-family: inherit;
  font-size: var(--tblr-body-font-size);
  margin: 0;
  max-width: 70ch;
  white-space: pre-wrap;
}

.message-source {
  background-color: var(--tblr-bg-surface-tertiary);
  border: 1px solid var(--tblr-border-color);
  border-radius: var(--tblr-border-radius);
  color: var(--tblr-body-color);
  font-family: var(--tblr-font-monospace);
  margin: 0;
  max-height: 70vh;
  overflow: auto;
  padding: 1rem;
  tab-size: 2;
}
</style>
