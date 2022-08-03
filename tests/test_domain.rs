extern crate mes;

use crate::mes::*;
use std::fs;

#[cfg(test)]
mod mes_tests {
    use std::{result, borrow::Borrow, fs::create_dir};

    /*
    use crate::mes::RawMedo;

    #[test]
    fn test_parse() {
        let dummy = crate::mes::MedoPiece {
            dialogue: "あ、キタキタ。女の子二人を待たせるなんて、失礼だぞ。".to_string(),
            comments: "場所は駅前（＃はコメント全般のデコレーター）".to_string(),
            sound_note: "駅前の音（＄は音響指示のデコレーター）".to_string(),
            charactor: "ニカ（＠はキャラ名のデコレーター）".to_string(),
            sound_position: "正面（！はサウンドポジション）".to_string()
        };
        let text = std::fs::read_to_string("tests/SampleMimeyScript.txt").unwrap();
        //println!("{}", text);
        let result = crate::mes::parseMedoBody(&text);
        println!("{:?}", result.pieces);
        
        assert_eq!(result.pieces[0],dummy)
    }

    #[test]
    fn test_toflatDialogueString(){
        let dummy = "
#第一章
#第一場
#二人の寝室

$りんごを剥く音
想太「秋葉、りんご剥いたよ」

秋葉「ありがとう、想太さん」

想太「具合の方はどうだい？」

秋葉「だいぶ良くなりました」
";
        let result = RawMedo::toflat_DialogueString(dummy);
        println!("{}",result);
        assert!(true);
    }

    #[test]
    fn test_flatDialogue(){
        let text = std::fs::read_to_string("tests/AfterTheMonday/CommonScript.txt").unwrap();
        let mut rawMedo = crate::mes::parseRawMedo(&text);
        rawMedo.toflat_Dialogue();
        let res = rawMedo.parseToMedo();
        //println!("{:?}", res.body.pieces);
        
        for ele in res.body.pieces {
            println!("\"{}\",\"{}\",\"{}\",\"{}\"", ele.charactor,ele.dialogue,ele.comments,ele.sound_note);            
        }
        
    }
    */
    #[test]
    fn test_parseMes(){
        /*
        let dummy = crate::mes::MedoPiece {
            dialogue: "あ、キタキタ。女の子二人を待たせるなんて、失礼だぞ。".to_string(),
            comments: "場所は駅前（＃はコメント全般のデコレーター）".to_string(),
            sound_note: "駅前の音（＄は音響指示のデコレーター）".to_string(),
            charactor: "ニカ（＠はキャラ名のデコレーター）".to_string(),
            sound_position: "正面（！はサウンドポジション）".to_string()
        };
         */
        let text = std::fs::read_to_string("tests/SampleMimeyScript.txt").unwrap();
        //println!("{}", text);
        let result = crate::mes::parseMeSToJson(&text);
        println!("{}", result);
        
        //assert_eq!(result.body.pieces[0],dummy)
    }
    
    //TODO: Medoヘッダーがただしくパースされていないっぽいのでテストを書く
    /*
    #[test]
    fn test_parseRawMedo(){
        let text = std::fs::read_to_string("tests/SampleScript.txt").unwrap();
        let result = crate::mes::parseRawMedo(&text);
        //println!("RawMedo.body=>{:?}", result.parseBody().unwrap());
        assert_eq!(1, 1);
        //println!("RawMedo.header=>{:?}", result.getHeader().unwrap());
    }
     */
}