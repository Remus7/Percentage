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

async function findIngredient{
    
}
</script>

<template>

<input class="input" v-model="Ingredient" placeholder="Type ingredient name">
<button class="butAdd" @click="addIngredient">Add Ingredient</button>

<button class="butNewIngredient" v-for="ingredient in Ingredients">
<button @click="removeIngredient(index)" v-for="(ingredient, index) in Ingredients">
    {{ ingredient }}
</button>

</template>

<style>
.input{
    position:absolute;
    top:117px;
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
</style>
