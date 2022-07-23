<template>
  <q-page class="justify-center">
    <div style="max-width: 500px;">
      <q-input
        type="textarea"
        v-model="text"
        autogrow
        @update:model-value="parser"
      ></q-input>
    </div>
    <div>
      {{ result }}
    </div>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import init, { parseMeSToJson } from 'mes/mes';
import { parse } from 'path';

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
      result.value = "now loading wasm..."
    })
    const result = ref('');
    init().then(()=>{
      console.log(parseMeSToJson(text.value))
      parser.value = (text: string) => {
        result.value = parseMeSToJson(text)
      }
      parser.value(text.value)
    })

    return {
      text,
      parser,
      result
    };
  }
});
</script>
