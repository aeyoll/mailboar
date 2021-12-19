import Vue from 'vue';

// Axios
import instance from './plugins/axios';
import VueAxios from 'vue-axios';
Vue.use(VueAxios, instance);

// Components
import Components from './components/index';
Object.keys(Components).forEach(key => {
  Vue.component(`V${key}`, Components[key]);
});

// Vuex
import Vuex from 'vuex';
Vue.use(Vuex);

const apiAddress = document.getElementById('app').dataset.apiAddress ?? '127.0.0.1:1080';

const store = new Vuex.Store({
  state: {
    apiAddress: `http://${apiAddress}`,
  },
});

// Router
import VueRouter from 'vue-router';
Vue.use(VueRouter);

import IndexPage from './pages/IndexPage.vue';
import MessagePage from './pages/MessagePage.vue';

const routes = [
  { path: '/', component: IndexPage, name: 'index' },
  { path: '/messages/:id', component: MessagePage, name: 'message' },
];

const router = new VueRouter({
  mode: 'history',
  routes,
});

// eslint-disable-next-line no-unused-vars
const app = new Vue({
  router,
  store,
  el: '#app',
});
