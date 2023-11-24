<router-view :key="$route.path" />
<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks, AvailableDrinks, Ingredients } from "../App.vue";
import "../assets/search.css";

const Ingredient: Ref<string> = ref("");

const areDrinks = ref(false); //catches if there are drinks possible to be served
const debugMsg = ref("");
const showSuggestions: Ref<boolean> = ref(false);
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
  showSuggestions.value = false;
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
  
  if (Ingredient.value == "") {
    showSuggestions.value = false;
  }
}

function showMoreDrinks(): void {
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

  <!-- The autocomplete list -->
  <div v-if="showSuggestions" class="autocomplete-list">
    <div class="divAutofill" v-for="suggestion in filteredSuggestions()">
      <button class="autocomplete-item" @click="selectSuggestion(suggestion)">
        {{ suggestion }}
      </button>
    </div>
  </div>
</template>