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

// Router
import VueRouter from 'vue-router';
Vue.use(VueRouter);

import IndexPage from './pages/IndexPage.vue';

const routes = [
  { path: '/', component: IndexPage },
];

const router = new VueRouter({
  routes,
});

// eslint-disable-next-line no-unused-vars
const app = new Vue({
  router,
  el: '#app',
});
