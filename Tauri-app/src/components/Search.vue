<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Ingredients: Ref<string[]> = ref([]);
const Ingredient = ref("");

const areDrinks = ref(true); //catches if there are drinks possible to be served
const AvailableDrinks: Ref<string[]> = ref([]);
const favoriteDrinks: Ref<string[]> = ref([]);
const debugMsg = ref("");

AvailableDrinks.value = ["Vodka", "Margarita", "Tequila"];

async function testButton() {
  try {
    AvailableDrinks.value = await invoke("drink_from_ingredients", {
      ingredientVec: Ingredients.value,
    });
  } catch (error) {
    debugMsg.value = error as string;
  }
}
function removeIngredient(index: Number) {
  Ingredients.value.splice(index, 1);
}

async function addIngredient() {
  Ingredients.value.push(Ingredient.value);
}

// function addFavorite(){
//     favoriteDrinks.value.push()
// }
</script>
<template>
  <p>{{ debugMsg }}</p>
  <input
    class="input"
    v-model="Ingredient"
    placeholder="Type ingredient name"
  />
  <button class="butAdd" @click="addIngredient">Add Ingredient</button>
  <button @click="testButton">test button</button>

  <button class="ingredient-item" v-for="(ingredient, index) in Ingredients">
    {{ ingredient }}
    <button class="remove" @click="removeIngredient(index)">X</button>
  </button>

  <p v-if="!areDrinks">There is no drink available!</p>
  <br />

  <button
    v-cloak="drink - button"
    v-for="(drink, index) in AvailableDrinks"
    class="drink-button"
  >
    {{ drink }}
    <!-- <button class="favorite" @click="addFavorite(index)">&hearts;</button> -->
    
  </button>
</template>

<style scoped>
.ingredient-item {
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

.drink-button {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
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
</style>
