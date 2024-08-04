import EmptyState from './empty-state';
import MessageList from './message-list';
import MessageListItem from './message-list-item';
import MessageDetailDefinition from './message-detail-definition';
import MessageAttachments from './message-attachments';
import MessageHtml from './message-html';
import AppHeader from './app-header';
import SendMessageModal from './send-message-modal';
import ToastNotifications from './toast-notifications';

export default Object.assign({},
  EmptyState,
  AppHeader,
  MessageList,
  MessageListItem,
  MessageDetailDefinition,
  MessageAttachments,
  MessageHtml,
  SendMessageModal,
  ToastNotifications,
);
