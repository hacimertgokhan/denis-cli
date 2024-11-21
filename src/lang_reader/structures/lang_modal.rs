#[derive(serde::Deserialize, Debug, Default)]
pub struct LanguageModal {
    #[serde(rename = "monitoring-started")]
    pub monitoring_started: String,

    #[serde(rename = "max-token-size")]
    pub max_token_size: String,

    #[serde(rename = "starts-with-details")]
    pub starts_with_details: String,

    #[serde(rename = "invalid-parametre")]
    pub invalid_parametre: String,

    #[serde(rename = "language-changed")]
    pub language_changed: String,

    #[serde(rename = "token_creation_error")]
    pub token_creation_error: String,

    #[serde(rename = "bye")]
    pub bye: String,

    #[serde(rename = "token_created_successfuly")]
    pub token_created_successfuly: String,

    #[serde(rename = "cli-help")]
    pub cli_help: String,

    #[serde(rename = "exiting_memory_monitoring")]
    pub exiting_memory_monitoring: String,

    #[serde(rename = "created_by")]
    pub created_by: String,

    #[serde(rename = "startup-information")]
    pub startup_information: Vec<String>,

    #[serde(rename = "startup-swd")]
    pub startup_swd: Vec<String>,

    #[serde(rename = "startup-port-and-token-information")]
    pub startup_port_and_token_information: Vec<String>,

    #[serde(rename = "client-connected")]
    pub client_connected: String,

    #[serde(rename = "waiting-for-client-connection")]
    pub waiting_for_client_connection: String,

    #[serde(rename = "invalid-denis-mode")]
    pub invalid_denis_mode: String,

    #[serde(rename = "denis-global-logger")]
    pub denis_global_logger: String,

    #[serde(rename = "listen-memory")]
    pub listen_memory: Vec<String>,

    #[serde(rename = "help")]
    pub help: Vec<String>,
}
