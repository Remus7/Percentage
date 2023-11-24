<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks } from "../App.vue";
import "../assets/favorite.css"

const ingredientList: Ref<String[]> = ref([])
const activ: Ref<boolean> = ref(false);

const glassText: Ref<string> = ref("");
const imageUrl: Ref<string> = ref("");
const preparationText: Ref<string> = ref("");
const debugMsg: Ref<string> = ref("");
const currentDrink = ref("");

function removeFavorite(index: number): void{
    favoriteDrinks.value.splice(index, 1);
}

const drinkDetails: Ref<string[]> = ref([]);

async function getDetails(drink: string) {
  if(drink != currentDrink.value){
    currentDrink.value = drink;
    debugMsg.value = "";
    activ.value = true;

    imageUrl.value = "";
    glassText.value = "Loading ..."
    preparationText.value = "Loading ..."

    try{
      drinkDetails.value = await invoke ("get_ingredients", {drink: drink});
      imageUrl.value = await invoke("get_details", {drink: drink, requestType: 0});
      glassText.value = await invoke("get_details", {drink: drink, requestType: 1});
      preparationText.value = await invoke("get_details", {drink: drink, requestType: 2});
    } catch (e) {
      debugMsg.value = e as string;
    }
  }
}
</script>

<template>

<button class="favorite-drink" data-toggle="dropdown" v-for="(drink, index) in favoriteDrinks" :key="drink" @mouseover="getDetails(drink)">
    {{ drink }}:
    <!-- <div v-for="(ingredient, index) in drinkDetails">{{ ingredient}}, &nbsp;</div> -->
    <div class="dropdown">
  <button id="asdf">&#9432;</button>
  <div class="dropdown-content">
    <button class="dropdown-square" data-toggle="dropdown" id="asd"><u>Ingredients</u>: 
      <select v-model="ingredientList">
        <option v-for="(detail, detailIndex) in drinkDetails" :key="detailIndex">{{ detail }}</option>
      </select>
    </button>
    <button class="dropdown-square" id="asd"><u>Glass type</u>: {{ glassText }}</button>
    <button class="dropdown-square" id="asd"><u>Preparation</u>: {{ preparationText }}</button>
    <button class="dropdown-square" id="asd"><img :src="imageUrl" /></button>
  </div>
</div>
    <button class="remove" @click="removeFavorite(index)">X</button>
</button>

</template>