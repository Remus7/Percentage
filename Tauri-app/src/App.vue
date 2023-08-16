<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, Ref } from "vue";
import { RouterLink, RouterView } from 'vue-router'

</script>

<script lang="ts">
export const favoriteDrinks: Ref<string[]> = ref([]);
const agePrompt: Ref<boolean> = ref(true);

export async function ageConfirm() {
  agePrompt.value = false;
};
export async function ageCancel() {
  agePrompt.value = false;
  window.close();
};
</script>

<template>
  <v-dialog v-if="agePrompt" max-width="400" class = "container">
    <v-card>
      <v-card-title class="titleAge">Confirm Age</v-card-title>
      <br>
      <v-card-text class="textAge" >
        Are you over 18 years old?
      </v-card-text>
      <v-card-actions class="button-container">
        <v-btn class="close-button" color="primary" @click="ageConfirm">Yes</v-btn>
        <v-btn class="close-button" color="secondary" @click="ageCancel">No</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <div v-else class="container">
    <div class="navbar">
     <RouterLink class="links" to="/"> <img class="navImage" src="/images/home_icon.png" alt="Home"> </RouterLink>
     <RouterLink class="links" to="/search"><img class="navImage" src="/images/search_icon.png" alt="Search"></RouterLink>
     <RouterLink class="links" to="/favorite"><img class="navImage" src="/images/fav_icon.png" alt="Favourites"></RouterLink>
     <a class="links" href="https://github.com/Remus7/Percentage" target="_blank"><img class="navImage" src="/images/github_icon.png" alt="GitHub"> </a>
    </div>

    <RouterView />
  </div>
</template>

<style scoped>
.container {
  font-family: Arial, sans-serif;
  text-align: center;
  margin-top: 6%;
}

.titleAge {
  font-size: 45px;
  margin-bottom: 50px;
  color:black;
}
.textAge {
  font-size: 30px;
  margin-bottom: 30px;
  color: black;
}

.button-container {
  display: flex;
  justify-content: center;
  margin-top: 20px;
}

.close-button {
  display: flex;
  text-align: center, center;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  margin-left: 40px;
  margin-right: 40px;
  width: 50px;
  padding: 8px;
  border: 1px solid white;
  background-color: #0E4749;
  color: white;
  border-radius: 4px;
}

.close-button:hover {
  background-color: #0056b3;
}

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
