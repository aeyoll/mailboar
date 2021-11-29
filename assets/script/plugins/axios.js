import Vue from 'vue';
import axios from 'axios';
import axiosRetry from 'axios-retry';

import VueProgressBar from 'vue-progressbar';
Vue.use(VueProgressBar, {
  color: '#35cbcc',
  failedColor: 'red',
  height: '2px',
});

const app = new Vue();

const instance = axios.create({
  timeout: 10000,
  headers: {'X-Requested-With': 'XMLHttpRequest'},
});

axiosRetry(instance, {
  retries: 3,
  shouldResetTimeout: true,
  retryCondition: (error) => axiosRetry.isNetworkOrIdempotentRequestError(error) || error.code === 'ECONNABORTED',
});

instance.interceptors.request.use(config => {
  app.$Progress.start();
  return config;
});

instance.interceptors.response.use(response => {
  app.$Progress.finish();
  return response;
});

window.axios = instance;

export default instance;
