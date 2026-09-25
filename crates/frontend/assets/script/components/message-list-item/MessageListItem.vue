<template>
  <router-link
    :to="{ name: 'message', params: { id: message.id } }"
    class="message-list-item"
    exact-active-class="active"
  >
    <span class="avatar avatar-sm message-list-item-avatar">{{ senderInitial }}</span>

    <span class="message-list-item-body">
      <span class="message-list-item-subject">{{ subject }}</span>

      <span class="message-list-item-line message-list-item-route">
        <span class="message-list-item-addresses">{{ from }} → {{ to }}</span>
        <abbr class="message-list-item-stamp mb-numeric" :title="formattedDate">{{ relativeDate }}</abbr>
      </span>
    </span>
  </router-link>
</template>

<script>
import { emailMixin } from '../../mixins/email';

export default {
  name: 'MessageListItem',
  mixins: [emailMixin],
  props: {
    message: {
      type: Object,
      default() {
        return {};
      },
    },
  },
};
</script>

<style scoped lang="scss">
  // Mail arrives over SSE, so a row entering the list is the one moment in the
  // app worth animating. Deliberately no fill-mode: if the animation never
  // runs the row still renders at its resting style.
  @keyframes message-arrives {
    from {
      opacity: 0;
      transform: translateY(-.25rem);
    }
  }

  .message-list-item {
    animation: message-arrives .3s ease-out;
    background-color: var(--tblr-bg-surface-tertiary);
    border-bottom: 1px solid var(--tblr-border-color);
    color: inherit;
    display: flex;
    gap: .75rem;
    padding: .875rem 1rem;
    text-decoration: none;
    transition: background-color .12s ease-out;
  }

  @media (prefers-reduced-motion: reduce) {
    .message-list-item {
      animation: none;
      transition: none;
    }
  }

  // Tabler's tertiary surface collapses onto the card surface in dark, which
  // would leave the resting and selected rows identical.
  [data-bs-theme='dark'] .message-list-item {
    background-color: var(--tblr-bg-surface-secondary);
  }

  .message-list-item:hover {
    background-color: var(--tblr-bg-surface);
  }

  .message-list-item.active {
    background-color: var(--tblr-bg-surface);
    box-shadow: inset 1px 0 0 0 var(--mb-ember);
  }

  .message-list-item.active .message-list-item-avatar {
    --tblr-avatar-bg: var(--tblr-primary);
    color: var(--tblr-primary-fg);
  }

  .message-list-item-avatar {
    flex: none;
    font-weight: var(--tblr-font-weight-bold);
  }

  .message-list-item-body {
    display: grid;
    gap: .125rem;
    min-width: 0;
    width: 100%;
  }

  .message-list-item-line {
    align-items: baseline;
    display: flex;
    gap: .75rem;
    justify-content: space-between;
    min-width: 0;
  }

  .message-list-item-subject {
    color: var(--tblr-body-color);
    font-weight: var(--tblr-font-weight-bold);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .message-list-item-route {
    color: var(--tblr-secondary);
    font-size: .75rem;
  }

  .message-list-item-stamp {
    border: 0;
    flex: none;
    text-decoration: none;
    white-space: nowrap;
  }

  .message-list-item-addresses {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
