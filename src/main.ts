import { createApp } from 'vue';
import { createPinia } from 'pinia';

// @ts-expect-error Library related warning
import { RecycleScroller } from 'vue-virtual-scroller';

// Fonts
import '@fontsource/jetbrains-mono/400.css';
import '@fontsource/jetbrains-mono/800.css';

import 'material-icons/iconfont/outlined.css';
import 'material-symbols/outlined.css';

// Styles
import '@/styles/main.scss';
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css';

import App from './App.vue';
import router from './router';

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.component('RecycleScroller', RecycleScroller);

app.mount('#app');
