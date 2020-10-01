import Vue from 'vue';
import Component from 'vue-class-component';
import App from './app'

function main() {
  const vue = new Vue({
    el: "#app",
    template: `<div class="app">
          <h1>Hello Vue.js!</h1>
          <app message="My Counter for TypeScript"></app>
        </div>`,
    components: {
      'app': App,
    },
  });
}

main();
