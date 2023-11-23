<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, Ref } from "vue";
import { RouterLink, RouterView } from 'vue-router'
import "./assets/style.css";

</script>

<script lang="ts">
export const favoriteDrinks: Ref<string[]> = ref([]);
export const AvailableDrinks: Ref<string[]> = ref([]); //VECTORUL PRIMIT DIN RUST CU BAUTURILE AVAILABLE
export const Ingredients: Ref<string[]> = ref([]);
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