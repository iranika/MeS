use mes::Medo;
use serde::{Deserialize, Serialize};
use config::{Config};

macro_rules! if_wasm {
    ($($item:item)*) => {$(
        #[cfg(target_arch = "wasm32")]
        $item
    )*}
}

macro_rules! if_hyper {
    ($($item:item)*) => {$(
        #[cfg(not(target_arch = "wasm32"))]
        $item
    )*}
}

if_hyper! {
    pub mod mes;

    mod test_mes;

    #[inline(always)]
    pub fn parseMeSToJson(text: &str) -> String {
        let conf = mes::builder::new();
        let result = mes::parseMeSToJson(text, &conf);
        result
    }
    pub fn parseMeS(text: &str) -> Medo{
        let conf = mes::builder::new();
        let result = mes::parse_mes(text, &conf);
        result
    }
    #[inline(always)]
    pub fn getVTT(mes_text: &str) -> String{
        mes::getVTT(mes_text, &mes::builder::new())
    }
    
    #[inline(always)]
    pub fn countDialogueWordToJson(text: &str) -> String {
        let conf = mes::builder::new();
        mes::countDialogueWordToJson(text, &conf)
    }



    
}

if_wasm! {
    extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;
    pub use crate::mes::*; 
    pub mod mes;

    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    
    #[wasm_bindgen(getter_with_clone)]
    pub struct WasmMedo {
        medo: crate::mes::Medo
    }

    #[wasm_bindgen]
    #[derive(Debug, Deserialize, Serialize)]
    pub struct WasmMeSBuilder {
        builder: crate::mes::builder::MeSBuilder
    }



    #[wasm_bindgen]
    impl WasmMeSBuilder {
        #[wasm_bindgen]
        pub fn getDefaultConfig() -> String {
            serde_json::to_string(&mes::builder::new()).unwrap() 
        }

        pub fn showConfig(&self) -> String{
            serde_json::to_string(self).unwrap()
        }
    }

    
    #[wasm_bindgen]
    pub fn parseMeSToJson(text: &str) -> String {
        mes::builder::new().parse_to_jsonstr(text)
    }
    
    #[wasm_bindgen]
    pub fn parseMeSToJsonWithConf(text: &str, json: &str) -> String {
        mes::builder::set_json_conf(json).parse_to_jsonstr(text)
    }

    #[wasm_bindgen]
    pub fn getDefaultConfigJson()->String{
        serde_json::to_string(&mes::builder::new()).unwrap()
    }
    
    #[wasm_bindgen]
    pub fn countDialogueWordToJson(text: &str) -> String {
        mes::countDialogueWordToJson(text, &mes::builder::new())
    }

    #[wasm_bindgen]
    pub fn getVTT(text: &str) -> String {
        mes::getVTT(text, &mes::builder::new())
    }
    
    #[wasm_bindgen]
    pub fn echo(text: &str) -> String {
        text.to_string()
    }
    
}

