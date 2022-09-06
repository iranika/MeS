<template>
  <video-player
    :sources="source"
    controls
    :loop="true"
    :volume="0.6"
    :tracks="tracks"
    :audioTracks="tracks"
  />
</template>

<script>
  import { defineComponent } from 'vue'
  import { VideoPlayer } from '@videojs-player/vue'
  import 'video.js/dist/video-js.css'

  export default defineComponent({
    components: {
      VideoPlayer
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