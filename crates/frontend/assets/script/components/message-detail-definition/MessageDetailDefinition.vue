<template>
  <div class="message-definition">
    <table class="table card-table table-striped">
      <tbody>
        <tr v-for="(definition, index) in definitions" :key="index">
          <td class="label" width="110">
            {{ definition.label }}
          </td>
          <td>
            <span v-if="message && definition.value">{{ definition.value }}</span>
            <div v-else class="skeleton-line skeleton-line-full" />
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
import { emailMixin } from '../../mixins/email';

export default {
  name: 'MessageDetailDefinition',
  mixins: [emailMixin],
  props: {
    message: {
      type: Object,
      default() {
        return {};
      },
    },
  },
  computed: {
    definitions: function() {
      return [
        {'label': 'From', value: this.message ? this.from : null},
        {'label': 'To', value: this.message ? this.to : null},
        {'label': 'Subject', value: this.message ? this.subject : null},
      ];
    },
  },
};
</script>

<style scoped lang="scss">
.table {
  table-layout: fixed;
}

.label {
  font-weight: 700;
  text-align: right;
}
</style>
