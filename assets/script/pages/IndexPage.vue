<template>
  <v-message-list :messages="messages"></v-message-list>
</template>

<script>
export default {
  name: 'index-page',
  data: function () {
    return {
      messages: []
    };
  },
  mounted: function () {
    this
      .axios
      .get('http://127.0.0.1:1080/messages')
      .then((response) => {
        const messages = response.data;
        this.messages = messages;
      });
  }
};
</script>
