<script setup lang="ts">
import { ref, Ref } from "vue";
import { favoriteDrinks } from "../layouts/App.vue";
const ingredientList: Ref<String[]> = ref([])
const activ: Ref<boolean> = ref(false);

const glassText: Ref<string> = ref("");
const imageUrl: Ref<string> = ref("");
const preparationText: Ref<string> = ref("");
const debugMsg: Ref<string> = ref("");
const currentDrink = ref("");

function removeFavorite(index: number): void {
  favoriteDrinks.value.splice(index, 1);
}

const drinkDetails: Ref<string[]> = ref([]);

async function getDetails(drink: string) {
  if (drink != currentDrink.value) {
    currentDrink.value = drink;
    debugMsg.value = "";
    activ.value = true;
    // console.log(await invoke ("get_details", {drink: drink}));

    imageUrl.value = "";
    glassText.value = "Loading ..."
    preparationText.value = "Loading ..."
    try {
      const response = await fetch('http://localhost:3000/get_ingredients/vodka');
      const data = await response.json();


      drinkDetails.value = data["ingredients"]

      imageUrl.value = data["image"]

      glassText.value = data["glass"]

      preparationText.value = data["preparation"]
    } catch (e) {
      debugMsg.value = e as string;
    }
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

  <button class="favorite-drink" data-toggle="dropdown" v-for="(drink, index) in favoriteDrinks" :key="drink"
    @mouseover="getDetails(drink)">
    {{ drink }}:
    <!-- <div v-for="(ingredient, index) in drinkDetails">{{ ingredient}}, &nbsp;</div> -->
    <div class="dropdown">
      <button id="asdf">&#9432;</button>
      <div class="dropdown-content">
        <button data-toggle="dropdown" id="asd"><u>Ingredients:</u>
          <select v-model="ingredientList">
            <option v-for="(detail, detailIndex) in drinkDetails" :key="detailIndex">{{ detail }}</option>
          </select>
        </button>
        <button class="dropdown-square" id="asd"><u>Glass type:</u>{{ glassText }} </button>
        <button class="dropdown-square" id="asd"><u>Preparation:</u>{{ preparationText }}</button>
        <button class="dropdown-square" id="asd"><img :src="imageUrl" /></button>
      </div>
    </div>
    <button class="remove" @click="removeFavorite(index)">X</button>
  </button>
</template>

<style>
.dropdown-square {
  color: white;
}

.favorite-drink {
  background-color: #A3333D;
  color: white;
  margin-top: 10px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  padding: 8px;
  border-width: 2px;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.favorite-drink:active {
  /* opacity:0.8; */
  background-color: #7d262d;
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

#asd {
  height: 50px;
  width: 100%;
}

#asdf {
  background-color: #F64740;
  border-width: 1px;
  border-style: solid;
  border-color: #ccc;
}

.dropdown-square {
  color: black;

}
</style>