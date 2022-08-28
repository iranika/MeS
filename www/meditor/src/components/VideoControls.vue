<template>
  <div>
    <video playsinline width="500" height="100" controlslist="nofullscreen">
      <source src="../assets/通話中si8.mp3" type="audio/mp3" />
      <track label="日本語" kind="subtitles" srclang="ja" src="./通話中si8.vtt" type="text/vtt" default />
    </video>

    <ul id="video-controls" class="controls">
      <li><button id="playpause" type="button">Play/Pause</button></li>
      <li><button id="stop" type="button">Stop</button></li>
      <li class="progress">
        <progress id="progress" value="0" min="0">
          <span id="progress-bar"></span>
        </progress>
      </li>
      <li><button id="mute" type="button">Mute/Unmute</button></li>
      <li><button id="volinc" type="button">Vol+</button></li>
      <li><button id="voldec" type="button">Vol-</button></li>
      <li><button id="fs" type="button">Fullscreen</button></li>
    </ul>
  </div>
</template>

<script lang="ts">
  import { defineComponent } from 'vue'
  export default defineComponent({
    // name: 'PageName'
    setup(){

      const video = document.getElementById
    }
  })
  </script>
  
<style>

</style>