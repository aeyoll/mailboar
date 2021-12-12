<template>
  <dl>
    <div v-for="(definition, index) in definitions" :key="index" class="row">
      <dt class="col-sm-4">
        {{ definition.label }}
      </dt>
      <dd class="col-sm-8">
        <span v-if="message && definition.value">{{ definition.value }}</span>
        <div v-else class="skeleton-line skeleton-line-full" />
      </dd>
    </div>
  </dl>
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
      ];
    },
  },
};
</script>
