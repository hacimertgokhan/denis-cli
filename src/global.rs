use lazy_static::lazy_static;
use std::fs;
use serde::Deserialize;
use std::sync::RwLock;

// LanguageModal ve read_global_toml fonksiyonu
#[derive(Deserialize, Debug)]
pub struct LanguageModal {
    pub language: String,
    pub description: String,
}

// JSON dosyasını okuma fonksiyonu
pub fn read_global_toml() -> Result<LanguageModal, Box<dyn std::error::Error>> {
    let file_content = fs::read_to_string("./global.toml")?;
    let language_modal: LanguageModal = toml::de::from_str(&file_content)?;
    Ok(language_modal)
}

// lazy_static ile global bir değişken oluşturuyoruz
lazy_static! {
    pub static ref CLI_LANGUAGE: RwLock<Option<LanguageModal>> = RwLock::new(None);
}

pub fn initialize_cli_language() {
    let mut cli_language = CLI_LANGUAGE.write().unwrap();
    *cli_language = read_global_toml().ok();
}
