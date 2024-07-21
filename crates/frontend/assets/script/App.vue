<template>
  <div class="wrapper">
    <header class="navbar navbar-expand-md d-print-none" data-bs-theme="dark">
      <div class="container-xl">
        <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbar-menu">
          <span class="navbar-toggler-icon" />
        </button>
        <h1 class="navbar-brand navbar-brand-autodark d-none-navbar-horizontal pe-0 pe-md-3">
          <router-link :to="{ name: 'index' }">
            Mailboar
          </router-link>
        </h1>
      </div>
    </header>
    <div class="page-wrapper">
      <div class="container-xl">
        <router-view />
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'App',
};
</script>
