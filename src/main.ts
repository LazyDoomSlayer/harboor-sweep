import { createApp } from 'vue';
import { createPinia } from 'pinia';

// Fonts
import '@fontsource/funnel-sans';

import 'material-icons/iconfont/outlined.css';
import 'material-symbols/outlined.css';
import 'material-symbols/rounded.css';

// Styles
import '@/styles/main.scss';
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css';

import App from './App.vue';
import router from './router';

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount('#app');
