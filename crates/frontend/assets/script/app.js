import { createApp } from 'vue';
import App from './App.vue';

const app = createApp(App);

// Axios
// ----------------------------------------------------------------------------
import axios from 'axios';
import axiosRetry from 'axios-retry';
import VueAxios from 'vue-axios';

const instance = axios.create({
  timeout: 10000,
  headers: {'X-Requested-With': 'XMLHttpRequest'},
});

axiosRetry(instance, {
  retries: 3,
  shouldResetTimeout: true,
  retryCondition: (error) => axiosRetry.isNetworkOrIdempotentRequestError(error) || error.code === 'ECONNABORTED',
});

window.axios = instance;
app.use(VueAxios, instance);

// Components
// ----------------------------------------------------------------------------
import Components from './components/index';
Object.keys(Components).forEach(key => {
  app.component(`V${key}`, Components[key]);
});

// Vuex
// ----------------------------------------------------------------------------
const apiUrl = document.getElementById('app').dataset.apiUrl ?? 'http://127.0.0.1:1080';

import { createStore } from 'vuex';
const store = createStore({
  state () {
    return {
      apiUrl: apiUrl,
      messages: [],
    };
  },
  mutations: {
    setMessages(state, messages) {
      state.messages = messages;
    },
    addMessage(state, message) {
      state.messages.push(message);
    },
    clearMessages(state) {
      state.messages = [];
    },
  },
  actions: {
    async addMessage({ commit }, message) {
      commit('addMessage', message);
    },
    async fetchMessages({ commit }) {
      const response = await app.axios.get(`${apiUrl}/messages`);

      if (response.status !== 200) {
        throw new Error('Failed to fetch messages');
      }

      commit('setMessages', response.data);
    },
    async deleteMessages({ commit }) {
      const response = await app.axios.delete(`${apiUrl}/messages`);

      if (response.status !== 204) {
        throw new Error('Failed to delete messages');
      }

      commit('clearMessages');
    },
  },
});
app.use(store);

// Router
// ----------------------------------------------------------------------------
import { createRouter, createWebHistory } from 'vue-router';
import IndexPage from './pages/IndexPage.vue';
import MessagePage from './pages/MessagePage.vue';

const routes = [
  { path: '/', component: IndexPage, name: 'index' },
  { path: '/messages/:id', component: MessagePage, name: 'message' },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});
app.use(router);

app.mount('#app');
