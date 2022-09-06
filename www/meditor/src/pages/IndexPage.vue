<template>
  <q-page class="flex justify-center">
    <q-card style="width: 98%; margin-bottom: 30px;">
      <q-card-section>
        <q-list>
          <q-expansion-item
            label="台本ビューの表示/非表示"
            style="text-align: center;"
            default-opened
            expand-separator
          >
            <PrintDaihon :pieces="result.body.pieces" :showConf="showConf"></PrintDaihon>
            <details>
            <summary style="text-align: left;">表示設定▼</summary>
              <q-checkbox size="xs" label="ト書き" v-model="showConf.showComment"></q-checkbox>
              <q-checkbox size="xs" label="人物" v-model="showConf.showChara"></q-checkbox>
              <q-checkbox size="xs" label="セリフ" v-model="showConf.showSerifu"></q-checkbox>
              <q-checkbox size="xs" label="文字数" v-model="showConf.showWordNum"></q-checkbox>
              <q-checkbox size="xs" label="タイミング情報" v-model="showConf.showTiming"></q-checkbox>
            </details>
          </q-expansion-item>
        </q-list>
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
            v-model="editorStore.db.text"
            autogrow
            filled
            @update:model-value="parser"
          ></q-input>
        </div>
        <div>※テキストは自動で保存されます。</div>
        <div>MeS記法の詳細は右記を参照してください→<a href="https://iranika.github.io/mesdoc/" target="_blank">https://iranika.github.io/mesdoc/</a></div>
        <details>
          <summary>エディタの操作パネル▼</summary>
          <div style="margin-top:5px;">
            <q-btn size="xs" color="primary" label="テキストを初期化する" @click="editorStore.initEditorStore()"></q-btn>          
          </div>
        </details>
      </q-card-section>
      <q-separator></q-separator>
      <q-card-section>
        <div class="text-h6">各種情報エリア</div>
        <details open>
          <summary style="text-align: left;">キャラ別のセリフ文字数集計▼</summary>
          <q-card-section>
            <table border="">
              <th>キャラクター</th>
              <th>セリフ文字数</th>
              <tbody>
                <tr v-for="v in counter" :key="v">
                  <td>{{ v.charactor }}</td>
                  <td>{{ v.word_count }}</td>  
                </tr>
              </tbody>
            </table>
            <div>カウント除外の文字：{{ ignore_char.length == 0 ? "なし" : ignore_char}}</div>
            <details>
              <summary><div>生データ（JSON）▼</div></summary>
              <div>{{ counter }}</div>
            </details>
          </q-card-section>
        </details>
        <q-separator></q-separator>
        <details>
          <summary style="text-align: left;">(WIP)コンフィグの設定▼</summary>
          <q-card-section>
            <q-input
              type="textarea"
              v-model="config"
              autogrow
              filled
              disable
            ></q-input>
          </q-card-section>
        </details>
        <q-separator></q-separator>
        <details>
          <summary style="text-align: left;">デバッグ情報▼</summary>
          <q-card-section>
            <div>パース情報</div>
            <div>
              {{ result }}
            </div>
          </q-card-section>
        </details>
        <q-separator></q-separator>
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
</style>

<script lang="ts">
import { defineComponent, ref } from 'vue';
import init, { parseMeSToJson, countDialogueWordToJson, getDefaultConfigJson, parseMeSToJsonWithConf } from 'mes/mes';
import PrintDaihon from '../components/PrintDaihon.vue';
import { useEditorStore } from 'stores/editorStore';

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
  components: { PrintDaihon },
  setup () {
    
    const editorStore = useEditorStore();
    const config = ref('');

    const parser = ref((val: string)=>{
      console.log('now loading wasm...')
    })
    const result = ref(<Medo>{
      body:{
        pieces:<MedoPiece[]>[]
      }
    });

    const counter = ref({});
    const ignore_char = ref('')

    init().then(()=>{
      console.log(parseMeSToJson(editorStore.db.text))
      parser.value = (text: string) => {
        result.value = JSON.parse(parseMeSToJson(editorStore.db.text))
        editorStore.commitEditorStore()
        counter.value = 
          Object.values(JSON.parse(countDialogueWordToJson(editorStore.db.text))) //JSON.parse(countDialogueWordToJson(editorStore.db.text))
      }
      config.value = JSON.stringify(JSON.parse(getDefaultConfigJson()), null, 5)
      ignore_char.value = JSON.parse(config.value).count_config.ignore_char ?? 'なし'
      console.log(config.value)
      parser.value(editorStore.db.text)
    })

    function reverse<T>(arr: Array<T>):Array<T>{
      if(arr.length === 0) return arr;
      var copy = arr.slice();
      return copy.reverse();
    }
    const showConf = ref({
      showChara: true,
      showSerifu: true,
      showWordNum: true,
      showComment: true,
      showDialogue: true,
      showTiming: false
    })

    return {
      editorStore,
      config,
      parser,
      result,
      reverse,
      showConf,
      counter,
      ignore_char
    };
  }
});
</script>
