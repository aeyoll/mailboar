import {
  formatDate,
  formatRelativeDate,
  formatSize,
  formatSubject,
  parseEmail,
  parseEmails,
  senderInitial as initialFrom,
} from '../helpers/email';

export const emailMixin = {
  computed: {
    from: function () {
      return parseEmail(this.message.sender);
    },
    to: function() {
      return parseEmails(this.message.recipients);
    },
    formattedDate: function () {
      return formatDate(this.message.created_at);
    },
    relativeDate: function () {
      return formatRelativeDate(this.message.created_at);
    },
    subject: function () {
      return formatSubject(this.message.subject);
    },
    senderInitial: function () {
      return initialFrom(this.from);
    },
    humanSize: function () {
      return formatSize(this.message.size);
    },
  },
};
