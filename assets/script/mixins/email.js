import { parseEmail } from '../helpers/email';

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime);

export const emailMixin = {
  computed: {
    from: function () {
      return parseEmail(this.message.sender);
    },
    to: function() {
      return this.message.recipients.map(recipient => parseEmail(recipient)).join(', ');
    },
    formattedDate: function () {
      return dayjs(this.message.created_at).format('DD MMMM YYYY, hh:mm:ss');
    },
    relativeDate: function () {
      return dayjs().to(dayjs(this.message.created_at));
    },
  },
};
