pub mod mes;

mod test_mes;

#[cfg(not(target_arch = "wasm32"))]
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
    native::parseMeSToJson(text)
}

#[inline(always)]
pub fn countDialogueWordToJson(text: &str) -> String {
    native::countDialogueWordToJson(text)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn countDialogueWordToJsonWithConf(text: &str, mesconf: MeSConfig) -> String {
    native::countDialogueWordToJsonWithConf(text.to_string(), mesconf)
}