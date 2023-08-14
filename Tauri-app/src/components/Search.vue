<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Ingredients: Ref<string[]> = ref([]);
const Ingredient = ref("");

async function addIngredient(){
    Ingredients.value.push(Ingredient.value);
}

function removeIngredient(index: Number){
    Ingredients.value.splice(index, 1);
}
</script>

<template>

<input v-model="Ingredient" placeholder="">
<button @click="addIngredient">Add Ingredient</button>

<button @click="removeIngredient(index)" v-for="(ingredient, index) in Ingredients">
    {{ ingredient }}
</button>

</template>
