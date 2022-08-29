<template>
  <video class="video-js" controls>
    <source src="/通話中si8.mp3" type="audio/mp3" />
    <track kind="captions" src="/通話中si8.vtt" srclang="ja" label="日本語" default>
  </video>
</template>

<script>
import { defineComponent } from 'vue'
import 'video.js/dist/video-js.css'

export default defineComponent({
  components: {
    
  },
  setup(){
    return {
      tracks: [{
        label: '日本語',
        kind: 'captions',
        srclang: 'ja',
        src: '/通話中si8.vtt',    
      }],
      source: [{
        src: '/通話中si8.mp3',
        type: 'audio/mp3'
      }]
    }
  }
})
</script>