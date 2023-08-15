<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, Ref } from "vue";
import { RouterLink, RouterView } from 'vue-router'
</script>

<script lang="ts">
export const favoriteDrinks: Ref<string[]> = ref([]);
</script>

<template>
  <div class="container">
    <div class="navbar">
     <RouterLink class="links" to="/"> <img src="/fav_icon.png" alt="Go Fuck Yourself"> </RouterLink>
     <RouterLink class="links" to="/search">Search</RouterLink>
     <RouterLink class="links" to="/favorite">Favorite</RouterLink>
    </div>

    <RouterView />
  </div>
</template>

<style scoped>

.imgg{
  height: 4000px;
  width: 4000px;
}

.navbar{
  height: 40px;
  background-color: #1EAF90;
  overflow:hidden;
  padding-top:12px;
  margin-top: 25px;
}
.links{
  color: white;
  text-align: center;
  padding:10px;
  /* text-decoration: none; */
  font-size: 17px;
}

.links:hover{
  opacity:0.7;
  /* color:black; */
}



</style>
