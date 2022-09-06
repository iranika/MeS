import {reactive} from 'vue';
import init, { parseMeSToJson } from 'mes/mes';

//import axios from 'axios';
/*
export class Hoge {
    private static 
}
*/

const initText = `
＠ニカ（＠はキャラ名のデコレーター）
＃場所は駅前（＃はコメント全般のデコレーター）
＄駅前の音（＄は音響指示のデコレーター）
！正面（！はサウンドポジション）
＆00:00:00.000 --> 00:00:06.000 
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

`

export class EditorStore {
    private static instance: EditorStore;
    
    private keys = {
        text: 'Text',
        setting: 'Setting',
        config: 'Config'
    }

    public db = reactive({
        text: String(window.localStorage.getItem(this.keys.text) ?? this.getInitText() ),
        setting: String(window.localStorage.getItem(this.keys.setting) ?? ''),
        config: String(window.localStorage.getItem(this.keys.config) ?? '')
    })
    public parser = reactive((val: string)=>{
        console.log('now loading wasm...')
    })
    public medo = reactive({

    })
  

    public toBoolean(boolstr: string): boolean{
        return boolstr.toLowerCase() === 'true';
    }

    public commitEditorStore(){
        window.localStorage.setItem(this.keys.text, this.db.text)
        window.localStorage.setItem(this.keys.config, this.db.config)
    }

    private getInitText(){
        return initText;
    }

    public initEditorStore(){
        this.db.text = this.getInitText();
        this.commitEditorStore();
        //.localStorage.setItem(this.keys.text, this.db.text)
    }

    public initMeS(){
        init().then(()=>{
            this.parser = (text: string) =>{
                this.medo = parseMeSToJson
            }
        })
    }

    // eslint-disable-next-line @typescript-eslint/no-empty-function
    private constructor(){}

    public static get getInstance():EditorStore{
        if (!this.instance){
            // eslint-disable-next-line @typescript-eslint/unbound-method
            this.instance = new EditorStore();
        }
        return this.instance;
    }
}

export function useEditorStore():EditorStore{
    return EditorStore.getInstance
}