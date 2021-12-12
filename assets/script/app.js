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

// Filters
Vue.filter('capitalize', function (value) {
  if (!value) {
    return '';
  }

  value = value.toString();
  return value.charAt(0).toUpperCase() + value.slice(1);
});

import IndexPage from './pages/IndexPage.vue';
import MessagePage from './pages/MessagePage.vue';

const routes = [
  { path: '/', component: IndexPage },
  { path: '/messages/:id', component: MessagePage, name: 'message' },
];

const router = new VueRouter({
  routes,
});

// eslint-disable-next-line no-unused-vars
const app = new Vue({
  router,
  el: '#app',
});
