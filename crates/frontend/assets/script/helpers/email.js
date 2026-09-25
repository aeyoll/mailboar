import { filesize } from 'filesize';

import dayjs from 'dayjs';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime);

/**
 * Extract the address from an envelope value. SMTP senders reach us as raw
 * envelope strings that may carry parameters ("<a@b.test> size=814") or no
 * angle brackets at all, so anything unbracketed is returned as-is.
 */
export const parseEmail = (rawEmail) => {
  const address = (rawEmail ?? '').match(/<(.*?)>/);

  return address ? address[1] : (rawEmail ?? '').trim();
};

export const parseEmails = (rawEmails) =>
  (rawEmails ?? []).map(parseEmail).join(', ');

export const formatSubject = (subject) => subject || '(no subject)';

export const senderInitial = (from) => {
  const letter = (from ?? '').match(/[a-z0-9]/i);

  return letter ? letter[0].toUpperCase() : '?';
};

export const formatSize = (bytes) => {
  const n = Number(bytes);

  return Number.isFinite(n) ? filesize(n) : '';
};

export const formatDate = (iso) => dayjs(iso).format('DD MMMM YYYY, HH:mm:ss');

export const formatRelativeDate = (iso) => dayjs().to(dayjs(iso));
