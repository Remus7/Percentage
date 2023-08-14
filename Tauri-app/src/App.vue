<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Home from "./components/Home.vue";
import Search from "./components/Search.vue";
import Favorite from "./components/Favorite.vue";
</script>

<template>

<<<<<<< HEAD
    <div clsas="navbar">
=======
    <div class="navbar">
>>>>>>> ff1baba25773c9095b65b142fde26b4f43326f47
     <RouterLink class="links" to="/">Home </RouterLink>
     <RouterLink class="links" to="/search">Search</RouterLink>
     <RouterLink class="links" to="/favorite">Favorite</RouterLink>
    </div>


  <div class="container">

    <RouterView />
  </div>
</template>

<style scoped>

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
