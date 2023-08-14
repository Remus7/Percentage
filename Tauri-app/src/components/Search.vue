<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const Ingredients: Ref<string[]> = ref([]);
const Ingredient = ref("");

async function addIngredient(){
    Ingredients.value.push(Ingredient.value);
}
</script>

<template>

<input v-model="Ingredient" placeholder="">
<button @click="addIngredient">Add Ingredient</button>

<button v-for="ingredient in Ingredients">
    {{ ingredient }}
</button>

</template>
