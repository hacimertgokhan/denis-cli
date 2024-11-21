use serde_json;
use std::fs;
use crate::global::CLI_LANGUAGE;
mod structures { pub mod lang_modal;}
use structures::lang_modal::LanguageModal;

pub fn read_json_file() -> Result<LanguageModal, Box<dyn std::error::Error>> {
    let language = CLI_LANGUAGE.read().unwrap();
    match language.as_ref() {
        Some(language) => {
            let file_content = fs::read_to_string(format!("./lang/{}.json", language.language))?;
            let language_modal: LanguageModal = serde_json::from_str(&file_content)?;
            Ok(language_modal)
        }
        None => {
            let default_language_modal = LanguageModal {
                monitoring_started: "Başlatıldı".to_string(),
                max_token_size: "1024".to_string(),
                starts_with_details: "Detaylarla başlar".to_string(),
                invalid_parametre: "Geçersiz parametre".to_string(),
                language_changed: "Dil değişti".to_string(),
                ..Default::default()
            };
            Ok(default_language_modal)
        }
    }
}
