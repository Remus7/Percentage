import { createRouter, createWebHistory } from 'vue-router';

// Import components
import Home from './components/Home.vue';

// Create routes
const routes =  [{ path: '/', component: Home}]

const router = createRouter({ routes, history: createWebHistory(import.meta.env.BASE_URL)})

export default router;