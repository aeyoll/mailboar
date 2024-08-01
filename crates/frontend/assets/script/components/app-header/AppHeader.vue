<template>
  <div class="page-header mb-3">
    <div class="row align-items-center">
      <div class="col">
        <div class="page-title">
          You have {{ messages.length }} message<template v-if="messages.length > 1">s</template>
        </div>
      </div>
      <div v-if="messages.length > 0" class="col-auto ms-auto">
        <div class="btn-list">
          <a href="#" class="btn" @click.prevent="localDeleteMessages()">
            <svg  xmlns="http://www.w3.org/2000/svg"  width="24"  height="24"  viewBox="0 0 24 24"  fill="none"  stroke="currentColor"  stroke-width="2"  stroke-linecap="round"  stroke-linejoin="round"  class="icon icon-tabler icons-tabler-outline icon-tabler-trash"><path stroke="none" d="M0 0h24v24H0z" fill="none"/><path d="M4 7l16 0" /><path d="M10 11l0 6" /><path d="M14 11l0 6" /><path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" /><path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" /></svg>
            Delete all messages ({{ messages.length }})
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { mapActions } from 'vuex';
export default {
  name: 'AppHeader',
  props: {
    messages: {
      type: Array,
      default() {
        return [];
      },
    },
  },
  methods: {
    ...mapActions(['deleteMessages']),
    localDeleteMessages() {
      this.deleteMessages();
      this.$router.push({'name': 'index'});
    },
  },
};
</script>
