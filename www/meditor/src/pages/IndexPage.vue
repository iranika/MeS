<template>
  <q-page class="flex justify-center">
    <q-card style="width: 98%;">
      <q-card-section>
        <table border="1">
          <tr class="">
            <td class="tate tr-sound" v-for="piece in reverse(result.body.pieces)" :key="piece"><div v-html="[piece.comments,piece.sound_note,piece.sound_position].filter(v=> v != '').join('<br>')"></div></td>
            <td class="tate tr-sound"><div>ト書き</div></td>
          </tr>
          <tr class="">
            <td class="tate tr-chara" v-for="piece in reverse(result.body.pieces)" :key="piece"><div>{{ piece.charactor }}</div></td>
            <td class="tate tr-chara"><div>人物</div></td>
          </tr>
          <tr class="tr-serifu">
            <td class="tate tr-serifu" v-for="piece in reverse(result.body.pieces)" :key="piece"><div>{{ piece.dialogue }}</div></td>
            <td class="tate tr-serifu"><div>セリフ</div></td>
          </tr>
        </table>
        <!--
        <div class="fit row reverse no-wrap justify-start items-start content-end" style="overflow-x: auto;">
          <div>
            <div class="tate tr-sound">ト書き</div>
            <div class="tate tr-chara">人物</div>
            <div class="tate tr-serifu">セリフ</div>
            <div class="tr-count">文字数</div>
          </div>
          <div v-for="piece in result.body.pieces" :key="piece" style="resize: both;">
            <div class="tate tr-sound font-geneikoburi">{{ piece.comments }}<br> {{ piece.sound_note }} <br> {{ piece.sound_position }} <br></div>
            <div class="tate tr-chara font-geneikoburi">{{ piece.charactor }}</div>
            <div class="tate tr-serifu font-geneikoburi">{{ piece.dialogue }}</div>
            <div class="tr-count">{{ piece.dialogue.length }}</div>
          </div>
        </div>
        -->
      </q-card-section>
      <q-separator></q-separator>
      <q-card-section>
        <div style="">
          <q-input
            type="textarea"
            v-model="text"
            autogrow
            filled
            @update:model-value="parser"
          ></q-input>
        </div>
      </q-card-section>
      <q-separator></q-separator>
      <q-card-section>
        <div>パース情報</div>
        <div>
          {{ result }}
        </div>
      </q-card-section>
    </q-card>
  </q-page>
</template>

<style scoped>
.tr-serifu{
  height: 250px;
  max-height: 300px;
  width: 100%;
  overflow-wrap:normal;
}
.tr-chara{
  height: 100px;
  max-height: 300px;
  width: 100%;
  overflow-wrap: break-word;
}
.tr-count{
  height: 50px;
  max-height: 300px;
  width: 100%;
  overflow-wrap: break-word;
}
.tr-sound{
  height: 150px;
  max-height: 300px;
  width: 100%;
  overflow-wrap: break-word;
}
.tate {
  writing-mode: vertical-rl;
  text-combine-upright: 3;
  text-orientation: upright;
  text-align: left;
}
</style>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import init, { parseMeSToJson } from 'mes/mes';


interface MedoPiece {
  dialogue: string;
  comments: string;
  sound_note: string;
  charactor: string;
  sound_position: string;
}
interface Medo {
  header: {
    raw: string;
  }
  body: {
    pieces: MedoPiece[]
  }
}

export default defineComponent({
  name: 'IndexPage',
  components: { },
  setup () {
    
    const text = ref(`
＠ニカ（＠はキャラ名のデコレーター）
＃場所は駅前（＃はコメント全般のデコレーター）
＄駅前の音（＄は音響指示のデコレーター）
！正面（！はサウンドポジション）
あ、キタキタ。女の子二人を待たせるなんて、失礼だぞ。

＠こいと
！正面
そういうニカちゃんも、ついさっき来たばかりじゃないですか。

＠ニカ
＄少し離れてヒソヒソ声になる
こういう時は待たせた弱みに漬け込んで、ランチを奢らせるのがだな…

＠こいと
前回もそうやって奢らせてたじゃないですか。今日はだめです。
＃ちょっと叱る感じで（デコレーターは後置もOK）

＠こいと
それにしても久しぶりですね。この三人で会うのは何年ぶりでしたっけ？

＠ニカ
確か、最後に会ったのは富山に海鮮丼食べに行ったときだから、二年ぶりだね。

`)
    const parser = ref((val: string)=>{
      console.log("now loading wasm...")
    })
    const result = ref(<Medo>{
      body:{
        pieces:<MedoPiece[]>[]
      }
    });
    init().then(()=>{
      console.log(parseMeSToJson(text.value))
      parser.value = (text: string) => {
        result.value = JSON.parse(parseMeSToJson(text))
      }
      parser.value(text.value)
    })

    function reverse<T>(arr: Array<T>):Array<T>{
      if(arr.length === 0) return arr;
      var copy = arr.slice();
      return copy.reverse();
    }

    return {
      text,
      parser,
      result,
      reverse
    };
  }
});
</script>
