<router-view :key="$route.path" />
<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks, AvailableDrinks, Ingredients } from "../App.vue";

const Ingredient: Ref<string> = ref("");

const areDrinks = ref(false); //catches if there are drinks possible to be served
const debugMsg = ref("");
const showSuggestions: Ref<boolean> = ref(true);
const suggestions: Ref<string[]> = ref([]);
const showMore: Ref<boolean> = ref(false);

async function SearchDrink() {
  try {
    AvailableDrinks.value = await invoke("drink_from_ingredients", {
      ingredientVec: Ingredients.value,
    });
    areDrinks.value = true;
  } catch (error) {
    areDrinks.value = false;
    debugMsg.value = error as string;
  }
}
function removeIngredient(index: number): void {
  Ingredients.value.splice(index, 1);
}

async function addIngredient() {
  Ingredients.value.push(Ingredient.value);
  Ingredient.value = "";
}

async function addFavorite(drink: string) {
  if (favoriteDrinks.value.indexOf(drink) === -1)
    favoriteDrinks.value.push(drink);
}
function filteredSuggestions() {
  return suggestions.value.filter((suggestion) =>
    suggestion.toLowerCase().startsWith(Ingredient.value)
  );
}

function selectSuggestion(suggestion: string) {
  Ingredient.value = suggestion;
  showSuggestions.value = false;
}
async function handleInput() {
  debugMsg.value = "";

  try {
    suggestions.value = await invoke("get_possible_ingredients");
    showSuggestions.value = true;
  } catch (e) {
    debugMsg.value = e as string;
  }
}
// function clearSuggestions() {
//   showSuggestions.value = false;
// }

function showMoreDrinks(): void{
  showMore.value = true;
}

</script>

<template>
  <p>{{ debugMsg }}</p>
  <div class="autocomplete-container">
  <input
    class="input"
    v-model="Ingredient"
    placeholder="Type ingredient name"
    @input="handleInput"
    v-on:keyup.enter="addIngredient"
  />
  <!-- The autocomplete list -->
  <div v-if="showSuggestions" class="autocomplete-list">
    <div v-for="suggestion in filteredSuggestions()">
      <button class="autocomplete-item" @click="selectSuggestion(suggestion)">
        {{ suggestion }}
      </button>
    </div>
  </div> 
  </div>
  <button class="butAdd" @click="addIngredient">Add Ingredient</button>
  <button class="searchbut" @click="SearchDrink">Search for drinks</button>

  <button class="ingredient-item" v-for="(ingredient, index) in Ingredients">
    {{ ingredient }}
    <button class="remove" @click="removeIngredient(index)">X</button>
  </button>
  
  <div class="drink-list-container">
  <div v-for="(drink, index) in AvailableDrinks">
    <button v-if="index < 5 || showMore === true" class="drink-button">
      <span class="drink-name">{{ drink }}</span>
      <button class="favorite" @click="addFavorite(drink)">	&#127864;</button>
    </button>  
  </div>  
  </div>
  <button v-if="areDrinks" @click="showMoreDrinks">Show more</button>

</template>
<style scoped>
.ingredient-item {
  float:left;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  padding: 8px;
  border: 1px solid white;
  background-color: #0e4749;
  color: white;
  border-radius: 4px;
}
.ingredient-item:hover {
  opacity: 0.8;
}
.remove {
  text-align: center;
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

.drink-list-container {
  max-height: 300px; /* Set the maximum height for the scrollable container */
  overflow-y: auto; /* Enable vertical scrolling if content overflows */
}

.drink-button {
  color: white;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 5px;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: #A3333D;
  border-color: white;
  width: 100%; /* Make the buttons occupy the full width */
}
.drink-name {
  flex: 1; /* Expand to take remaining space */
  text-align: center; /* Center-align the text */
}

.drink-button:hover {
  opacity: 0.9;
}
.favorite {
  background-color: #f64740;
  color: red;
  border: none;
  border-radius: 50%;
  padding: 4px 8px;
  cursor: pointer;
  /* transition: background-color 2s ease-in-out; */
}
.favorite:hover {
  background-color: #d32f2f;
}
.searchbut {
  margin-top: 6px;
  margin-bottom: 25px;
  border-radius: 15px;
  box-shadow: 5px 5px black;
}
.searchbut:hover {
  opacity: 0.8;
  transition: 1.5 s;
}
.butAdd {
  margin-top: 4px;
  border-radius: 15px;
  box-shadow: 5px 5px black;
}
.butAdd:hover {
  opacity: 0.8;
  transition: 1.5 s;
}

input {
  padding: 10px;
  border: 2px solid #2bb4b4;
  border-radius: 5px;
  background-color: #f0f7f7;
  color: #333;
  font-size: 16px;
  transition: border-color 0.3s, background-color 0.3s;
}
.autocomplete-container {
  position: relative;
}

.input {
  padding: 8px;
  width: 100%;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.autocomplete-list {
  position: absolute;
  top: 100%;
  left: 0;
  z-index: 1;
  width: 100%; /* Set the width to match the input */
  margin-top: 4px; /* Add margin to avoid overlap */
  background-color: #fff;
  border: 1px solid #ccc;
  border-radius: 4px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  max-height: 200px;
  overflow-y: auto;
}

.autocomplete-item {
  padding: 8px 16px;
  display: block;
  text-align: left;
  cursor: pointer;
  border: none;
  background-color: transparent;
  width: 100%;
  color: black;
}

.autocomplete-item:hover {
  background-color: #f2f2f2;
}

.autocomplete-item:focus {
  outline: none;
  background-color: #f2f2f2;
}
</style>