<router-view :key="$route.path" />
<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks, AvailableDrinks, Ingredients } from "../App.vue";

const Ingredient: Ref<string> = ref("");

const areDrinks = ref(true); //catches if there are drinks possible to be served
const debugMsg = ref("");
const showSuggestions: Ref<boolean> = ref(true);
const suggestions: Ref<string[]> = ref([]);

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
function clearSuggestions() {
  showSuggestions.value = false;
}
</script>

<template>
  <p>{{ debugMsg }}</p>
  <input
    class="input"
    v-model="Ingredient"
    placeholder="Type ingredient name"
    @input="handleInput"
    v-on:keyup.enter="addIngredient"
    @blur="clearSuggestions"
  />
  <button class="butAdd" @click="addIngredient">Add Ingredient</button>
  <button class="searchbut" @click="SearchDrink">Search for drinks</button>

  <button class="ingredient-item" v-for="(ingredient, index) in Ingredients">
    {{ ingredient }}
    <button class="remove" @click="removeIngredient(index)">X</button>
  </button>

  <p v-if="!areDrinks">ingredients are invalid. No drink available.</p>
  <br />

  <button v-for="drink in AvailableDrinks" class="drink-button">
    {{ drink }}
    <button class="favorite" @click="addFavorite(drink)">&#127864;</button>
  </button>

  <!-- <input type="text" v-model="searchText" @input="handleInput" @blur="clearSuggestions" placeholder="Enter a word"> -->

  <!-- The autocomplete list -->
  <div v-if="showSuggestions" class="autocomplete-list">
    <div v-for="suggestion in filteredSuggestions()">
      <div class="autocomplete-item" @click="selectSuggestion(suggestion)">
        {{ suggestion }}
      </div>
    </div>
  </div>
</template>

<style scoped>
.ingredient-item {
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

.drink-button {
  color: white;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 5px;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  background-color: #a3333d;
  border-color: white;
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
  width: 90%;
  padding: 10px;
  border: 2px solid #2bb4b4;
  border-radius: 5px;
  background-color: #f0f7f7;
  color: #333;
  font-size: 16px;
  transition: border-color 0.3s, background-color 0.3s;
}
</style>
