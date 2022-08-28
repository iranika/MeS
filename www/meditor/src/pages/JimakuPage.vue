<template>
  <q-page padding>
    <div>
      <div class="text-h4">音声字幕実験ルーム</div>
      <div>
        ここは音声ファイルに字幕を付与する実験のための部屋です。<br>
      </div>
    </div>
    <div>
      <video-controls></video-controls>
    </div>
    <div>
      音声ファイルの著作権は桃色CODE様にあります。<br>
      iranikaは桃鳥様の許諾を得て利用させていただいております。<br>
      http://momoirocode.web.fc2.com/ <br>
    </div>
    <!-- content -->
  </q-page>
</template>

<style scoped>
video {
	width:100%;
	height:120px;
	border:1px solid #ccc;
	object-fit:initial;  /*WebKit用（追加されるテキストボックスが収まるように）*/
}
video::cue {  /*疑似要素「::cue」を使います*/
	font-size:20px; /*デフォルトのフォントサイズだと小さいので大きくしました*/
}
</style>

<script lang="ts">
import { defineComponent } from 'vue'
import VideoControls from 'src/components/VideoControls.vue';

export default defineComponent({
    components: { VideoControls }
})
</script>
