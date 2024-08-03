<template>
  <div id="send-message-modal" class="modal fade" tabindex="-1" aria-labelledby="send-message-modal-label" aria-hidden="true">
    <div class="modal-dialog">
      <form class="modal-content" @submit.prevent="sendMessage">
        <div class="modal-header">
          <h5 id="send-message-modal-label" class="modal-title">
            Send a message
          </h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" />
        </div>
        <div class="modal-body">
          <div class="mb-3">
            <label for="send-message-to" class="form-label">To</label>
            <input id="send-message-to" v-model="to" type="email" class="form-control" placeholder="Recipient's email">
          </div>
        </div>
        <div class="modal-footer">
          <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
            Cancel
          </button>
          <button type="submit" class="btn btn-primary">
            Send
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import Modal from 'bootstrap/js/dist/modal';

export default {
  name: 'SendMessageModal',
  props: {
    opened: {
      type: Boolean,
      default: false,
    },
  },
  emits: ['send-message', 'update:opened'],
  data: function () {
    return {
      modal: null,
      to: '',
    };
  },
  watch: {
    opened(newValue) {
      if (newValue) {
        this.to = '';
        this.modal.show();
      } else {
        this.modal.hide();
      }
    },
  },
  mounted() {
    const modalElement = document.getElementById('send-message-modal');

    if (modalElement) {
      modalElement.addEventListener('hidden.bs.modal', () => {
        this.$emit('update:opened', false);
      });
      this.modal = new Modal(modalElement);
    }
  },
  methods: {
    sendMessage() {
      this.$emit('send-message', {to: this.to});
    },
  },
};
</script>
