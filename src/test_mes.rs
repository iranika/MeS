
use crate::mes::*;
use std::fs;

#[cfg(test)]
mod mes_unit_tests {
    use std::{result, borrow::Borrow, fs::create_dir};
    #[test]
    fn test_counter(){
        let text = std::fs::read_to_string("tests/SampleMimeyScript.txt").unwrap();
        let result = crate::mes::countDialogueWordToJson(&text);
        println!("{}",result);
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
}