<template>
  <div style="overflow-x: auto;">
    <table border="1" class="font-geneikoburi">
      <tr v-show="showConf.showComment">
        <td class="tate tr-sound" v-for="piece in reverse(pieces)" :key="piece"><div v-html="[piece.comments,piece.sound_note,piece.sound_position].filter(v=> v != '').join('<br>')"></div></td>
        <td class="tate tr-sound"><div>ト書き</div></td>
      </tr>
      <tr v-show="showConf.showChara">
        <td class="tate tr-chara" v-for="piece in reverse(pieces)" :key="piece"><div>{{ piece.charactor }}</div></td>
        <td class="tate tr-chara"><div>人物</div></td>
      </tr>
      <tr v-show="showConf.showSerifu">
        <td class="tate tr-serifu" v-for="piece in reverse(pieces)" :key="piece"><div>{{ piece.dialogue }}</div></td>
        <td class="tate tr-serifu"><div>セリフ</div></td>
      </tr>
      <tr v-show="showConf.showWordNum">
        <td class="tr-count" style="text-align:center;" v-for="piece in reverse(pieces)" :key="piece"><div>{{ piece.dialogue.length }}</div></td>
        <td class="tate tr-count"><div>文字数</div></td>
      </tr>
    </table>
  </div>
</template>

<style>
.tr-serifu{
  height: 250px;
  max-height: 300px;
  width: 100%;
  overflow-wrap:normal;
  padding-top: 4px;
}
.tr-chara{
  height: 100px;
  max-height: 300px;
  width: 100%;
  overflow-wrap: break-word;
  padding-top: 4px;
}
.tr-count{
  height: 60px;
  max-height: 300px;
  width: 100%;
  overflow-wrap: normal;
  padding-top: 4px;
}
.tr-sound{
  height: 150px;
  max-height: 300px;
  width: 100%;
  overflow-wrap: break-word;
  padding-top: 4px;
}
.tate {
  writing-mode: vertical-rl;
  text-combine-upright: 3;
  text-orientation: upright;
  text-align: left;
}
@page {
  size: A4;
  font-family: "Helvetica Neue", Arial, "Hiragino Kaku Gothic ProN", "Hiragino Sans", Meiryo, sans-serif;
}
</style>

<script>
import { defineComponent } from 'vue';
import {Previewer} from 'paper-view'
import '../../node_modules/paper-view/dist/css/paper.css'

export default defineComponent({
  // name: 'ComponentName'
  props: {
    pieces: [],
    showConf: {}
  },
  methods:{
    paper(){
      const paged = new Previewer();
      let content = this.$refs.content;
      let render = this.$refs.render;
      paged.preview(content, render, []);
    },
    reverse(arr){
      if(arr.length === 0) return arr;
      var copy = arr.slice();
      return copy.reverse();
    }

  },
  mounted(){
    console.log('tsg', this.tsv);
    this.paper();
  },
})
</script>
