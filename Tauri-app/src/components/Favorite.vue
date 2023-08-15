<script setup lang="ts">
import { ref, Ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks } from "../App.vue";
const ingredientList: Ref<String[]> = ref([])
const activ: Ref<boolean> = ref(false);
function removeFavorite(index: number): void{
    favoriteDrinks.value.splice(index, 1);
}

const drinkDetails: Ref<string[]> = ref([]);

async function getDetails(drink: string) {
  activ.value= true;
  console.log(await invoke ("get_details", {drink: drink}));
  try{
  drinkDetails.value = await invoke ("get_details", {drink: drink});
  }catch{
    console.log("No ingredients found!");
  }
}

// async function showIngredientList(){
//   try{
//   console.log(activ.value);
//   console.log(ingredientList.value);
//   drinkDetails.value = await invoke ("get_details", {drink: drink});
//   console.log(ingredientList.value);
//   }catch{
//     console.log("No details found");
//   }
// }

</script>

<template>

<!-- <div class="dropdown">
  <button>Details</button>
  <div class="dropdown-content">
    <button >Button 1</button>
    <button>Button 2</button>
    <button>Button 3</button>
  </div>
</div> -->

<button class="favorite-drink" data-toggle="dropdown" v-for="(drink, index) in favoriteDrinks" :key="drink" @click="getDetails(drink)">
    {{ drink }}:
    <!-- <div v-for="(ingredient, index) in drinkDetails">{{ ingredient}}, &nbsp;</div> -->
    <div class="dropdown">
  <button id="asdf">&#9432;</button>
  <div class="dropdown-content">
    <button data-toggle="dropdown" id="asd">Ingredients
      <select v-model="ingredientList">
        <option v-for="(detail, detailIndex) in drinkDetails" :key="detailIndex">{{ detail }}</option>
      </select>
    </button>
    <button @click="showGlassType" id="asd">Glass type
    </button>
    <button @click="showPreparationMethod" id="asd">Preparation</button>
    <button @click="showImage" id="asd">Example image</button>
  </div>
</div>
    <button class="remove" @click="removeFavorite(index)">X</button>
</button>


</template>

<style>
.custom-button {
    display: inline-block;
    padding: 10px 20px;
    background-image: linear-gradient(to bottom, #847577, #1EAF90);
    color: #FFFFFF;
    border: none;
    border-radius: 5px;
    cursor: pointer;
    font-size: 16px;
    text-align: center;
    text-decoration: none;
    transition: background-color 0.3s ease, transform 0.2s ease;
  }

  .custom-button:hover {
    background-image: linear-gradient(to bottom, #0E4749, #002626);
    transform: scale(1.05);
  }
.favorite-drink {
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

.dropdown {
  position: relative;
  display: inline-block;
  border-radius: 0px;
}

.dropdown-content {
  border-radius: 0px;
  display: none;
  position: absolute;
  /* background-color: #1EAF90; */
  min-width: 20px;
  box-sizing: 20px 100px;
  z-index: 1;

}

.dropdown:hover .dropdown-content {
  display: block;
}
#asd{
 height:50px;
 width:100%;
}

</style>