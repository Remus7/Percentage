<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
</script>

<template>
<button class="custom">favorite exemplu
    <!-- <button class="fas fa-cocktail" @click="FUNCTIE CARE TE TRIMITE LA O PAGINA UNDE VEZI DETALII DESPRE BAUTURA RESPECTIVA">#f561</button> -->
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
</style>