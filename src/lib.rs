pub mod mes;

mod test_mes;
use native::MeSConfig;

#[cfg(all(target_os = "windows", not(target_arch = "wasm32")))]
use self::mes as native;

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
use self::wasm as native;

#[cfg(not(any(target_os = "windows", target_arch = "wasm32")))]
use self::mes as native;


#[inline(always)]
pub fn parseMeSToJson(text: &str) -> String {
    native::parse_mes_to_json(text)
}

#[inline(always)]
pub fn countDialogueWordToJson(text: &str) -> String {
    native::count_dialogue_word_to_json(text)
}

#[inline(always)]
pub fn countDialogueWordToJsonWithConf(text: &str, mesconf: MeSConfig) -> String {
    native::count_dialogue_word_to_json_with_conf(text.to_string(), mesconf)
}