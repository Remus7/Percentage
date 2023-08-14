<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Ingredients: Ref<string[]> = ref([]);
const AvailbleDrinks: Ref<string[]> = ref([]);
const Ingredient = ref("");
const areDrinks = ref(true); //catches if there are drinks possible to be served

async function addIngredient(){
    Ingredients.value.push(Ingredient.value);
    try{
    AvailbleDrinks.value = await invoke ("drink_from_ingredients", {ingredients: Ingredients})
    }catch(error){
        isDrinks.value = false;
    }
}

function removeIngredient(index: Number){
    Ingredients.value.splice(index, 1);
}
</script>

<template>

<input class="input" v-model="Ingredient" placeholder="Type ingredient name">
<button class="butAdd" @click="addIngredient">Add Ingredient</button>

<button class="ingredient-item" v-for="(ingredient, index) in Ingredients">
    {{ ingredient }}
    <button class="remove" @click="removeIngredient(index)">X</button>
</button>

<p v-if="!areDrinks">There is no drink available with these ingredients!</p>

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
</style>


<!-- <style>
.input{
    position:absolute;
    top:200px;
   border-radius:0px;
    border-width:2px;
    border-color:#0E4749;
    width: 250px;
    /* border-radius:10px; */
}
.butNewIngredient{
    height: 35px;
    width: 200px;
    border-width:2px;
    border-color: #0E4749;
    border-radius: 0px;
}
.butAdd{
    border-width: 2px;
    border-radius:0px;
    border-color: #0E4749;
    width: 250px;
}
</style> -->
