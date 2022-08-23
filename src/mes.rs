pub mod builder;

use std::{iter::Iterator, vec, collections::HashMap};
extern crate regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use self::builder::FlatDialogueConfig;
use std::default;

/* MeSのコア処理 */
//NOTE: メンバを増減するときは、builder.rsのMedoPieceConfigも編集すること
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MedoPiece {
    pub dialogue: String,
    pub comments: String,
    pub sound_note: String,
    pub charactor: String,
    pub sound_position: String,
    pub timing: String
}

impl Default for MedoPiece {
    fn default() -> Self {
        Self {
            ..Default::default()
        }
    }
}

#[derive(Debug,PartialEq,Serialize, Deserialize)]
pub struct MedoBody {
    pub pieces: Vec<MedoPiece>,
}

#[derive(Debug,PartialEq,Serialize, Deserialize)]
pub struct MedoHeader {
    pub raw: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Medo {
    pub header: MedoHeader,
    pub body: MedoBody
}


#[derive(Debug,PartialEq,Clone,Serialize, Deserialize)]
pub struct RawMedo {
    pub header: String,
    pub body: String
}

impl RawMedo {
    //TODO: toflat->common スクリプトへの再変換ができるようにする

    pub fn doflat(&mut self, conf: &builder::MeSBuilder) -> RawMedo{
        //NOTE:　フラットレイヤー
        self.toflat_dialogue(conf);
        
        self.clone()
    }
    pub fn toflat_dialogue(&mut self, conf: &builder::MeSBuilder)-> RawMedo{
        self.body = RawMedo::toflat_dialogue_string(&self.body.as_str(), conf);
        return self.clone()
    }
    //TODO: toflat_Dialogueとロジックを共有する
    pub fn toflat_dialogue_string(text: &str, conf: &builder::MeSBuilder)-> String{
        let flat_dialogue_config = &conf.mes_config.flat_dialogue_config;
        let re = Regex::new(r"\n{3,}").unwrap();
        let name_re = Regex::new(format!("{}{}", r"^.*", flat_dialogue_config.start_str).as_str()).unwrap();
        let raw = re.replace_all(text, "\n\n").to_string();
        let line: Vec<&str> = raw.split("\n").collect();
        let body = line
            .into_iter()
            .map(|mut x| -> String
                {
                    match name_re.captures(x){
                        Some(val) => {
                            let name = val.get(0).unwrap().as_str().replace(flat_dialogue_config.start_str.as_str(), "");
                            let rep_name = name.clone() + flat_dialogue_config.start_str.as_str();
                            let dialogue = x.replace(&rep_name, "").replace(flat_dialogue_config.end_str.as_str(), "");

                            return format!("@{}\n{}\n", name, &dialogue)
                        },
                        None => x.to_string()
                    }
                }
            )
            .collect::<Vec<String>>()
            .join("\n");        
        return body
    }

}

impl RawMedo {
    pub fn parse_header(&self) -> Option<MedoHeader>{
        return Some(MedoHeader{
            raw : "".to_string()
        })
    }
    pub fn parse_body(&self) -> Option<MedoBody>{
        let result = parseMedoBody(self.body.as_str());
        //println!("{:?}", result);
        return Some(result)
    }
    pub fn parse_to_medo(&mut self) -> Medo{
        
        Medo{
            header: MedoHeader { raw: self.header.to_string() },
            body: parseMedoBody(&self.body)
        }
    }
}

impl MedoBody {
    fn get_attribute(block: Vec<&str>, prefix: Vec<char>) -> Vec<String> {
        let attrs: Vec<String> = block
            .into_iter()
            .filter(|x| prefix.iter().any(|&p| {
                match x.chars().nth(0){
                    Some(v) => v == p,
                    None => false
                }
            }))
            .map(|v| -> String {
                let mut text = v.to_string().clone();
                text.remove(0);
                text
            })
            .collect();
        //println!("{:?}", attrs);
        return attrs;
    }
    fn get_dialogue(block: Vec<&str>, ignore_prefix: Vec<char>) -> Vec<String> {
        let dialogue = block
            .into_iter()
            .filter(|x| ignore_prefix.iter().all(|&p| {
                match x.chars().nth(0){
                    Some(v) => v != p,
                    None => false
                }
            }))
            .map(|v| v.to_string())
            .collect();
        //println!("{:?}", dialogue);
        return dialogue;
    }
}


pub fn parseMeSToJson(text: &str) -> String{
    let medo = parse_mes(text);
    let json = serde_json::to_string(&medo).unwrap();
    return json
}

pub fn parse_mes(text: &str) -> Medo {
    //HeaderとBodyに分離
    let mut rawMedo = parseRawMedo(text);
    //CommonScript等の差異を均す
    rawMedo.doflat(&builder::new());
    //println!("{}",rawMedo.body);
    //rawMedo.body = RawMedo::toflat_DialogueString(&(rawMedo.body));
    //Headerのパース
    //Bodyのパース
    //HeaderとBodyをMedoに結合

    return Medo {
        header: rawMedo.parse_header().unwrap(),
        body: rawMedo.parse_body().unwrap()
    }
}

pub fn parseRawMedo(text: &str) -> RawMedo {
    let tmp = text.replace("\r\n", "\n");
    let blocks: Vec<&str> = tmp.split("----\n").collect();
    if blocks.len() == 1 {
        return RawMedo {
            header: "".to_string(),
            body: blocks[0].to_string()
        }
    }
    return RawMedo {
        header: blocks[0].to_string(),
        body: blocks[1].to_string()
    }
}

pub fn parseMedoBody(_text: &str) -> MedoBody {
    
    let tmp = _text.replace("\r\n", "\n");
    //TODO: blocksに不要な空白行から生成されているblockは削除するようにする
    //TODO: 空白行にスペース等があった場合のためにトリミングをする
    let blocks: Vec<&str> = tmp.split("\n\n").collect();
    //println!("blocks{:?}", blocks);
    let block = blocks
        .into_iter()
        .map(|x| -> MedoPiece {
            let lines: Vec<&str> = x.split("\n").collect::<Vec<&str>>();
            let dialogue = MedoBody::get_dialogue(
                lines.clone(),
                vec!['#', '＃', '$', '＄', '@', '＠', '!', '！']
            ).join("\n"); //MedoBody::get_comments(lines).join(",");
            let comments = MedoBody::get_attribute(lines.clone(), vec!['#', '＃']).join(","); //MedoBody::get_comments(lines).join(",");
            let sound_note = MedoBody::get_attribute(lines.clone(), vec!['$', '＄']).join(",");
            let charactor = MedoBody::get_attribute(lines.clone(), vec!['@', '＠']).join(",");
            //println!("{:?}", charactor);
            let sound_position = MedoBody::get_attribute(lines.clone(), vec!['!', '！']).join(",");
            
            return MedoPiece {
                dialogue: dialogue,
                comments: comments,
                sound_note: sound_note,
                charactor: charactor,
                sound_position: sound_position,
                ..Default::default()
            };
            //println!("{:?}",&result);
        })
        .collect();
    //println!("{:?}", block);

    let result: MedoBody = MedoBody { pieces: block };

    return result;
}


/* WordCount関連のコード */
#[derive(Debug,PartialEq,Serialize, Deserialize)]
pub struct WordCount{
    charactor: String,
    word_count: usize
}

pub fn countDialogueWordToJsonWithConf(mut text: String, conf: builder::MeSBuilder) -> String{
    conf.count_config.ignore_char.into_iter().for_each(|c|{
        text = text.replace(&c, "");
    });
    let result = countDialogueWordToJson(&text);
    return result
}

pub fn countDialogueWordToJson(text: &str) -> String{
    let medo = parse_mes(text);
    //キャラクター毎にワード数を集計する
    let mut word_counter: HashMap<String, WordCount> = HashMap::new(); 
    medo.body.pieces.into_iter().for_each(|piece: MedoPiece|{
        match word_counter.get_mut(&piece.charactor) {
            Some(x) => {
                //既存のきゃらの集計追加
                x.word_count += piece.dialogue.graphemes(true).count();
            }
            None => {
                //新規キャラの集計追加
                word_counter.insert(piece.charactor.clone(), WordCount { charactor: piece.charactor.clone(), word_count: piece.dialogue.graphemes(true).count() });
            }
        }
    });
    let json = serde_json::to_string(&word_counter).unwrap();
    return json;
}
