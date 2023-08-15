<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks } from "../App.vue";

const Ingredients: Ref<string[]> = ref([]);
const Ingredient = ref("");

const areDrinks = ref(true); //catches if there are drinks possible to be served
const AvailableDrinks: Ref<string[]> = ref([]);
const debugMsg = ref("");

AvailableDrinks.value = ["Vodka", "Margarita", "Tequila"];

async function SearchDrink(){
  try{
        AvailableDrinks.value = await invoke ("drink_from_ingredients", {ingredientVec: Ingredients.value})
    } catch(error){
        areDrinks.value = false;
        debugMsg.value = error as string;
    }
}
function removeIngredient(index: Number){
    Ingredients.value.splice(index, 1);
}

async function addIngredient() {
  Ingredients.value.push(Ingredient.value);
}

function addFavorite(drink: string){ 
    favoriteDrinks.value.push(drink);                        
    console.log(favoriteDrinks.value);
}
</script>

<template>
  <p>{{ debugMsg }}</p>
  <input
    class="input"
    v-model="Ingredient"
    placeholder="Type ingredient name"
  />
  <button class="butAdd" @click="addIngredient">Add Ingredient</button>
  <button class="searchbut" @click="SearchDrink">Search for drinks</button>

  <button class="ingredient-item" v-for="(ingredient, index) in Ingredients">
    {{ ingredient }}
    <button class="remove" @click="removeIngredient(index)">X</button>
  </button>

  <p v-if="!areDrinks">There is no drink available!</p>
  <br />

<button v-for="drink in AvailableDrinks" class="drink-button">{{ drink }}
    <button class="favorite" @click="addFavorite(drink)">&hearts;</button>
</button>

</template>

<style scoped>
.ingredient-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  padding: 8px;
  border: 1px solid white;
  background-color: #0E4749;
  color: white;
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

.drink-button {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 2px;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: #a360b1;
}

.drink-button:hover {
  background-color: #45a049;
}
.favorite {
  background-color: #751414;
  color: red;
  border: none;
  border-radius: 50%;
  padding: 4px 8px;
  cursor: pointer;
  transition: background-color 0.3s ease-in-out;
}
.favorite:hover {
  background-color: #d32f2f;
}
.searchbut{
  margin-top:6px; 
  margin-bottom: 25px;
  border-radius: 15px;
  box-shadow: 5px 5px  black;
}
.searchbut:hover{
  opacity:0.8;
  transition:1.5 s;
}
.butAdd{
  margin-top:4px; ;
  border-radius: 15px;
  box-shadow: 5px 5px  black;
}
.butAdd:hover{
  opacity:0.8;
  transition:1.5 s;
}

input{
  width: 100%;
  padding: 10px;
  border: 2px solid #2BB4B4; 
  border-radius: 5px;
  background-color: #F0F7F7; 
  color: #333;
  font-size: 16px;
  transition: border-color 0.3s, background-color 0.3s;
}
</style>
