const mes = require("../pkg/mes");

const test = `
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

`

console.log("mes", mes.parseMeSToJson(test));
