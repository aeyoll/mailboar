<template>
  <table class="table table-striped">
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
          <a :href="store.apiAddress + attachment.href">{{ attachment.filename }}</a>
        </td>
        <td>
          {{ humanReadableFilesize(attachment.size) }}
        </td>
        <td>
          {{ attachment.type }}
        </td>
      </tr>
    </tbody>
  </table>
</template>

<script>
import filesize from 'filesize';
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
    apiAddress: state => state.apiAddress,
  }),
  methods: {
    humanReadableFilesize(size) {
      return filesize(size);
    },
  },
};
</script>
