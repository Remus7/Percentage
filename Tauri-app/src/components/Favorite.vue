<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks } from "../App.vue";

function removeFavorite(index: Number){
    favoriteDrinks.value.splice(index, 1);
}

const drinkDetails: Ref<string> = ref([]);

async function getDetails(drink: string) {
  console.log(await invoke ("get_details", {drink: drink}));
  try{
  drinkDetails.value = await invoke ("get_details", {drink: drink});
  }catch{
    console.log("No ingredients found!");
  }
}

</script>

<template>

<button class="favorite-drink" v-for="(drink, index) in favoriteDrinks" :key="drink" @click="getDetails(drink)">
    {{ drink }}
    {{ drinkDetails }}
    <button class="remove" @click="removeFavorite(index)">X</button>
</button>

</template>

<style>
.favorite-drink {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
}
.remove {
  background-color: #f44336;
  color: white;
  border: none;
  border-radius: 50%;
  padding: 4px 8px;
  cursor: pointer;
  transition: background-color 0.3s ease-in-out;
}
.remove:hover {
  background-color: #d32f2f;
}

</style>