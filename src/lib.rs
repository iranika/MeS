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
    #[cfg(not(target_arch = "wasm32"))]
    use native::builder;
    
    #[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
    use self::mes as native;
    #[inline(always)]
    pub fn parseMeSToJson(text: &str) -> String {
        native::parseMeSToJson(text)
    }
    
    #[inline(always)]
    pub fn countDialogueWordToJson(text: &str) -> String {
        native::countDialogueWordToJson(text)
    }
    
}

if_wasm! {
/*  extern crate wasm_bindgen;
    use wasm_bindgen::prelude::*;
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    #[cfg(feature = "wee_alloc")]
    #[global_allocator]
    static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT; */
    pub mod mes;
    pub mod wasm;
}

