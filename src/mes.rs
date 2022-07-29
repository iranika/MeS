use std::{iter::Iterator, vec, borrow::Borrow};
extern crate regex;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug,PartialEq,Serialize, Deserialize)]
pub struct MedoPiece {
    pub dialogue: String,
    pub comments: String,
    pub sound_note: String,
    pub charactor: String,
    pub sound_position: String,
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

    pub fn doflat(&mut self) -> RawMedo{
        //NOTE:　フラットレイヤー
        self.toflat_Dialogue();
        
        self.clone()
    }
    pub fn toflat_Dialogue(&mut self)-> RawMedo{
        let re = Regex::new(r"\n{3,}").unwrap();
        let nameRe = Regex::new(r"^.*「").unwrap();
        let raw = re.replace_all(self.body.as_str(), "\n\n").to_string();
        let line: Vec<&str> = raw.split("\n").collect();
        self.body = line
            .into_iter()
            .map(|mut x| -> String
                {
                    match nameRe.captures(x){
                        Some(val) => {
                            let name = val.get(0).unwrap().as_str().replace("「", "");
                            let repName = name.clone() + "「";
                            let dialogue = x.replace(&repName, "").replace("」", "");

                            return format!("@{}\n{}\n", name, &dialogue)
                        },
                        None => x.to_string()
                    }
                }
            )
            .collect::<Vec<String>>()
            .join("\n");
        
        return self.clone()
    }
    //TODO: toflat_Dialogueとロジックを共有する
    pub fn toflat_DialogueString(text: &str)-> String{
        let re = Regex::new(r"\n{3,}").unwrap();
        let nameRe = Regex::new(r"^.*「").unwrap();
        let raw = re.replace_all(text, "\n\n").to_string();
        let line: Vec<&str> = raw.split("\n").collect();
        let body = line
            .into_iter()
            .map(|mut x| -> String
                {
                    match nameRe.captures(x){
                        Some(val) => {
                            let name = val.get(0).unwrap().as_str().replace("「", "");
                            let repName = name.clone() + "「";
                            let dialogue = x.replace(&repName, "").replace("」", "");

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
    pub fn parseHeader(&self) -> Option<MedoHeader>{
        return Some(MedoHeader{
            raw : "".to_string()
        })
    }
    pub fn parseBody(&self) -> Option<MedoBody>{
        let result = parseMedoBody(self.body.as_str());
        //println!("{:?}", result);
        return Some(result)
    }
    pub fn parseToMedo(&mut self) -> Medo{
        
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
    fn get_dialogue(block: Vec<&str>, ignorePrefix: Vec<char>) -> Vec<String> {
        let dialogue = block
            .into_iter()
            .filter(|x| ignorePrefix.iter().all(|&p| {
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
    fn toflat_Dialogue(block: Vec<&str>) -> Vec<String>{
        //セリフ表記を均す
        /* e.g.

        ```
        にか「これがセリフじゃ」
        ```
        ↓　Common表記からMeS表記に変換
        ```
        @にか
        これがセリフじゃ
        ```
        */

        return vec!["".to_string()]
    }
}




pub fn parseMeSToJson(text: &str) -> String{
    let medo = parseMeS(text);
    let json = serde_json::to_string(&medo).unwrap();
    return json
}

pub(crate) fn parseMeS(text: &str) -> Medo {
    let dummy = MedoPiece {
        dialogue: "あ、キタキタ。女の子二人を待たせるなんて、失礼だぞ。".to_string(),
        comments: "comment text".to_string(),
        sound_note: "駅前の音".to_string(),
        charactor: "ニカ".to_string(),
        sound_position: "1".to_string()
    };
    //HeaderとBodyに分離
    let mut rawMedo = parseRawMedo(text);
    rawMedo.doflat();
    //println!("{}",rawMedo.body);
    //rawMedo.body = RawMedo::toflat_DialogueString(&(rawMedo.body));
    //Headerのパース
    //Bodyのパース
    //HeaderとBodyをMedoに結合

    return Medo {
        header: rawMedo.parseHeader().unwrap(),
        body: rawMedo.parseBody().unwrap()
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
            };
            //println!("{:?}",&result);
        })
        .collect();
    //println!("{:?}", block);

    let result: MedoBody = MedoBody { pieces: block };

    return result;
}
