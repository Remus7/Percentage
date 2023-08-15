import { createRouter, createWebHistory } from 'vue-router';

// Import components
import Home from './components/Home.vue';
import Search from './components/Search.vue';
import Favorite from './components/Favorite.vue';

// Create routes
const routes =  [{ path: '/', component: Home},
                 { path: '/search', component: Search},
                 { path: '/favorite', component: Favorite },
                 { path: '/github', beforeEnter() {location.href = "https://github.com/Remus7/Percentage"}, component: Home}
                ]

const router = createRouter({ routes, history: createWebHistory(import.meta.env.BASE_URL)})

export default router;