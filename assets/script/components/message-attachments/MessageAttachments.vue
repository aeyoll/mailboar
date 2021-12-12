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
          <a :href="'http://127.0.0.1:1080' + attachment.href">{{ attachment.filename }}</a>
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
  methods: {
    humanReadableFilesize(size) {
      return filesize(size);
    },
  },
};
</script>
