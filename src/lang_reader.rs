use serde_json;
use std::fs;
use crate::global::CLI_LANGUAGE;
mod structures { pub mod lang_modal;}
use structures::lang_modal::LanguageModal;

pub fn read_json_file() -> Result<LanguageModal, Box<dyn std::error::Error>> {
    let language = CLI_LANGUAGE.read().unwrap();
    match language.as_ref() {
        Some(_language) => {
            let file_content = fs::read_to_string("lang/tr.json")?;
            let language_modal: LanguageModal = serde_json::from_str(&file_content)?;
            Ok(language_modal)
        }
        None => {
            let file_content = fs::read_to_string("lang/tr.json")?;
            let language_modal: LanguageModal = serde_json::from_str(&file_content)?;
            Ok(language_modal)
        }
    }
}
