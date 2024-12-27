import { createApp } from "vue";
import { createPinia } from "pinia";

// Fonts
import "@fontsource/jetbrains-mono/400.css";
import "@fontsource/jetbrains-mono/800.css";

import "material-icons/iconfont/outlined.css";
import "material-symbols/outlined.css";

// Styles
import "@/styles/main.scss";

import App from "./App.vue";
import router from "./router";

const app = createApp(App);

app.use(createPinia());
app.use(router);

app.mount("#app");
