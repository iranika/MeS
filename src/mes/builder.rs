use std::default;

use serde::{Deserialize, Serialize};

use super::{parse_mes, RawMedo, parseRawMedo};
use crate::mes::{Medo};

/* MeS Config関連のコード */
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct MeSConfig{
    pub name: String,
    /// Default is "----\n"
    pub header_delimiter: String,
    pub flat_dialogue_config: FlatDialogueConfig
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct FlatDialogueConfig{
    pub start_str: String,
    pub end_str: String,
    
}


#[derive(Debug, Deserialize, Serialize, Default)]
pub struct CountConfig{
    pub ignore_char: Vec<String>
}
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ChatConfig{

}

#[derive(Debug, Deserialize, Serialize)]
pub struct MeSBuilder{
    pub mes_config: MeSConfig,
    pub count_config: CountConfig,
    pub chat_config: ChatConfig
}

impl Default for MeSBuilder {
    fn default() -> Self {
        //デフォルト設定の定義
        Self {
            mes_config: MeSConfig {
                header_delimiter: "----\n".to_string(),
                flat_dialogue_config: FlatDialogueConfig {
                    start_str: "「".to_string(),
                    end_str: "」".to_string(),
                    ..Default::default()
                },
                ..Default::default() //残りはDefaultのデフォルトをセットする
            },
            count_config: CountConfig {
                ignore_char: vec![],
                ..Default::default()
            },
            chat_config: ChatConfig {
                ..Default::default()

            }
        }
    }
}

impl MeSBuilder {
    fn parseRawMedo(self: &Self, text: &str) -> RawMedo{
        let tmp = text.replace("\r\n", "\n");
        let blocks: Vec<&str> = tmp.split(self.mes_config.header_delimiter.as_str()).collect();
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
}

impl MeSBuilder {
    pub fn parse(self: &Self, mes_text: &str) -> Medo{
        let mut rawMedo = self.parseRawMedo(mes_text);
        rawMedo.doflat(self);
        return Medo{
            header: rawMedo.parse_header().unwrap(),
            body: rawMedo.parse_body().unwrap()
        }
        //parse_mes(mes_text)

    }
    pub fn parse_to_jsonstr(self: &Self, mes_text: &str) -> String{
        let medo = self.parse(mes_text);
        serde_json::to_string(&medo).unwrap()
    }
}

pub fn new()->MeSBuilder{
    let builder:MeSBuilder = Default::default();
    builder
}



#[cfg(test)]
mod builder_test{

    use crate::mes::RawMedo;

    use super::MeSBuilder;

    //TODO: メジャーバージョンリリース時にテストデータを固定していく

    #[test]
    fn test_parseRawMedo(){
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let rawmedo: RawMedo = crate::builder::new().parseRawMedo(&text);

        println!("<header>{}</header>", rawmedo.header);
        println!("<body>{}</body>", rawmedo.body);
        
    }

    #[test]
    fn test_parse(){
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let medo = crate::builder::new().parse(&text);

        println!("<header>{:?}</header>", medo.header);
        println!("<body>{:?}</body>", medo.body);
        
    }

    #[test]
    fn test_parse_to_jsonstr(){
        let text = std::fs::read_to_string("tests/SampleCommonScript.txt").unwrap();
        let json = crate::builder::new().parse_to_jsonstr(&text);

        println!("{}", json);
    }

}