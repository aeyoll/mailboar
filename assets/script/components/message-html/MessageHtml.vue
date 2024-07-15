<template>
  <div class="message-html">
    <div class="message-html-iframe">
      <iframe
        id="html-content"
        frameborder="0"
        :height="htmlIframeHeight"
        style="width: 1px; min-width: 100%;"
      />
    </div>
  </div>
</template>

<script>
export default {
  name: 'MessageHtml',

  props: {
    parsedMessage: {
      type: Object,
      default() {
        return {};
      },
    },
  },

  data: function () {
    return {
      htmlIframeHeight: 0,
    };
  },

  mounted: function() {
    const htmlIframe = document.getElementById('html-content');

    let content = this.parsedMessage.html;

    if (content.indexOf('</body>') === -1) {
      content = '<body style="margin: 0; padding: 0;">' + content + '</body>';
    }

    htmlIframe.contentWindow.document.open();
    htmlIframe.contentWindow.document.write(content);
    htmlIframe.contentWindow.document.close();

    // Replace all inline images with their attachments
    htmlIframe.contentWindow.document.querySelectorAll('img').forEach(img => {
      if (/^cid:/.test(img.src)) {
        // replace with inline attachment
        const cid = img.src.substring(4).trim();
        const attachment = this.parsedMessage.attachments.find(attachment => attachment.contentId && attachment.contentId === `<${cid}>`);

        if (attachment) {
          img.src = URL.createObjectURL(new Blob([attachment.content], {type: attachment.mimeType}));
        }
      }
    });

    const sendPostMessage = () => {
      const height = htmlIframe.contentWindow.document.querySelector('body').offsetHeight;

      if (this.htmlIframeHeight !== height) {
        this.htmlIframeHeight = height;
      }
    };

    htmlIframe.contentWindow.onload = () => sendPostMessage();
    htmlIframe.contentWindow.onresize = () => sendPostMessage();

    htmlIframe.contentWindow.document.querySelectorAll('a').forEach(a => {
      a.setAttribute('target', '_blank');
    });
  },
};
</script>

<style scoped lang="scss">
.message-html-iframe,
.message-html-iframe iframe {
  width: 100%;
}
</style>
