<template>
  <table class="table table-vcenter">
    <thead>
      <tr>
        <th>Filename</th>
        <th>Size</th>
        <th>Type</th>
      </tr>
    </thead>
    <tbody>
      <tr v-for="attachment in attachments" :key="attachment.cid">
        <td>
          <a :href="apiUrl + attachment.href" class="message-attachment">
            <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="icon icon-tabler icons-tabler-outline icon-tabler-paperclip"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M15 7l-6.5 6.5a1.5 1.5 0 0 0 3 3l6.5 -6.5a3 3 0 0 0 -6 -6l-6.5 6.5a4.5 4.5 0 0 0 9 9l6.5 -6.5" /></svg>
            {{ attachment.filename }}
          </a>
        </td>
        <td class="mb-numeric text-muted">
          {{ formatSize(attachment.size) }}
        </td>
        <td class="text-muted">
          {{ attachment.type }}
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script>
import { formatSize } from '../../helpers/email';
import { mapState } from 'vuex';

export default {
  name: 'MessageAttachments',
  props: {
    attachments: {
      type: Array,
      default() {
        return [];
      },
    },
  },
  computed: mapState({
    apiUrl: state => state.apiUrl,
  }),
  methods: {
    formatSize,
  },
};
</script>

<style scoped lang="scss">
.message-attachment {
  align-items: center;
  display: inline-flex;
  gap: .375rem;
}

.icon {
  color: var(--tblr-secondary);
  height: 1rem;
  width: 1rem;
}
</style>
