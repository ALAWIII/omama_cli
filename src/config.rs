use omama_manager::chat::OConfig;

#[derive(Debug, Default)]
pub struct Config {
    pub delete_model: Option<String>,
    pub download_model: Option<String>,
    pub download_model_stream: Option<String>,
    pub fetch_model_by_name: Option<String>,
    pub install_tool: Option<String>, //*
    pub create_message: Option<OConfig>,
    pub get_all_messages: Option<i64>,
    pub get_chat_by_id: Option<i64>,
    pub get_summary_of_chat: Option<i64>,
    //-----------------------
    pub fetch_models_from_db: bool,
    pub fetch_models_from_web_to_db: bool,
    pub load_models_from_web_to_json: bool,
    pub load_models_from_json_file: bool,
    pub get_local_models_info: bool,
    pub is_installed_globally: bool,
    pub is_installed_locally: bool,
    pub is_ollama_running: bool,
    pub start_ollama_service: bool,
    pub create_chat: bool, // -> ID i64
    pub get_all_chats: bool,
}
impl Config {
    pub fn set_delete_model(&mut self, value: Option<String>) {
        self.delete_model = value;
    }

    pub fn set_download_model(&mut self, value: Option<String>) {
        self.download_model = value;
    }

    pub fn set_download_model_stream(&mut self, value: Option<String>) {
        self.download_model_stream = value;
    }

    pub fn set_fetch_model_by_name(&mut self, value: Option<String>) {
        self.fetch_model_by_name = value;
    }

    pub fn set_install_tool(&mut self, value: Option<String>) {
        self.install_tool = value;
    }

    pub fn set_create_message(&mut self, value: Option<OConfig>) {
        self.create_message = value;
    }

    pub fn set_get_all_messages(&mut self, value: Option<i64>) {
        self.get_all_messages = value;
    }

    pub fn set_get_chat_by_id(&mut self, value: Option<i64>) {
        self.get_chat_by_id = value;
    }

    pub fn set_get_summary_of_chat(&mut self, value: Option<i64>) {
        self.get_summary_of_chat = value;
    }

    pub fn set_fetch_models_from_db(&mut self, value: bool) {
        self.fetch_models_from_db = value;
    }

    pub fn set_fetch_models_from_web_to_db(&mut self, value: bool) {
        self.fetch_models_from_web_to_db = value;
    }

    pub fn set_load_models_from_web_to_json(&mut self, value: bool) {
        self.load_models_from_web_to_json = value;
    }

    pub fn set_load_models_from_json_file(&mut self, value: bool) {
        self.load_models_from_json_file = value;
    }

    pub fn set_get_local_models_info(&mut self, value: bool) {
        self.get_local_models_info = value;
    }

    pub fn set_is_installed_globally(&mut self, value: bool) {
        self.is_installed_globally = value;
    }

    pub fn set_is_installed_locally(&mut self, value: bool) {
        self.is_installed_locally = value;
    }

    pub fn set_is_ollama_running(&mut self, value: bool) {
        self.is_ollama_running = value;
    }

    pub fn set_start_ollama_service(&mut self, value: bool) {
        self.start_ollama_service = value;
    }

    pub fn set_create_chat(&mut self, value: bool) {
        self.create_chat = value;
    }

    pub fn set_get_all_chats(&mut self, value: bool) {
        self.get_all_chats = value;
    }
}
