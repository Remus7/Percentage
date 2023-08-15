<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, Ref } from "vue";
import { RouterLink, RouterView } from 'vue-router'

</script>

<script lang="ts">
export const favoriteDrinks: Ref<string[]> = ref([]);
const agePrompt: Ref<bool> = ref(false);

export async function ageConfirm() {
  agePrompt.value = false;
};
export async function ageCancel() {
  agePrompt.value = false;
  window.close();
};
</script>

<template>

  <div class="container">
    <div class="navbar">
     <RouterLink class="links" to="/"> <img class="navImage" src="/images/home_icon.png" alt="Home"> </RouterLink>
     <RouterLink class="links" to="/search"><img class="navImage" src="/images/search_icon.png" alt="Search"></RouterLink>
     <RouterLink class="links" to="/favorite"><img class="navImage" src="/images/fav_icon.png" alt="Favourites"></RouterLink>
     <RouterLink class="links" to="/github" ><img class="navImage" src="/images/github_icon.png" alt="GitHub"> </RouterLink>
    </div>

    <RouterView />
  </div>
</template>

<style scoped>

.navImage{
  height: 40px;
  width: auto;
  z-index: 1;
  color: white;
}

.navbar{
  height: 40px;
  background-color: #1EAF90;
  overflow: hidden;
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
.navbar{
  margin-bottom:50px;
}



</style>
