<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { favoriteDrinks } from "../App.vue";

function removeFavorite(index: Number){
    favoriteDrinks.value.splice(index, 1);
}

</script>
</template>
<button class="favorite-drink" v-for="(drink, index) in favoriteDrinks" :key="drink">
    {{ drink }}
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

</style>