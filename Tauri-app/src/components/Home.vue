<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

</script>

<template>
  <div class="animation-container">
    <svg class="animation-svg" xmlns="http://www.w3.org/2000/svg" 
        xmlns:xlink="http://www.w3.org/1999/xlink"
        xmlns:krita="http://krita.org/namespaces/svg/krita"
        xmlns:sodipodi="http://sodipodi.sourceforge.net/DTD/sodipodi-0.dtd"
        width="85pt"
        height="85pt"
        viewBox="0 0 85 85">
    <defs/>
    <
    <text id="shape0" transform="matrix(1 0 0 1 10 75)" fill="#ffffff" class="font_a" font-size="100" font-size-adjust="0.349432" font-stretch="normal"><tspan x="0">%</tspan></text>
    </svg>
  </div>
</template>


<style>
  .font_a {
    font-family: "Abyssinica SIL";
    src: url(../assets/font.ttf);
  }
  body {
    display: flex;
    flex-direction: column;
    justify-content: flex-start; /* Align content at the top */
    align-items: center;
    height: 100vh;
    margin: 0;
    background-color: #1EAF90;
  }

  .animation-container {
    position: relative;
    width: 500px;
    height: 500px;
    overflow: hidden;
  }

  .animation-svg {
    width: 100%;
    height: 100%;
    transform-origin: center center;
    animation: zoomOut 2s ease-in-out, idleAnim 2s ease-in-out infinite;
    animation-delay: 0s, 2s;
  }

  .change-yellow{
    fill: #B9A44C;
  }

  .change-blue{
    fill: #0C4767;
  }

  @keyframes zoomOut {
    0% {
      transform: scale(1);
    }
    100% {
      transform: scale(0.3);
    }
  }

  @keyframes idleAnim {
    0% {
      transform: scale(0.3) rotate(0deg);
    }
    25% {
      transform: scale(0.3) rotate( 20deg);
    }
    75% {
      transform: scale(0.3) rotate(-20deg);
    }
    100% {
      transform: scale(0.3) rotate(0deg);
    }
  }
</style>