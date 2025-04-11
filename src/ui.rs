use crate::Config;
use crate::parse_args;
use anyhow::Result;
use clap::{Arg, Command, command};
const HELP: &str = r#"
{before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading}
    {usage}

{all-args}{after-help}
"#;

pub fn get_args() -> Result<Config> {
    let app = command!()
        .author("allawiii")
        .about("Omama Manager")
        .next_line_help(true)
        .help_template(HELP)
        .subcommand(create_message_command())
        .subcommand(service_utils_command())
        .arg(
            Arg::new("create_chat")
                .long("create-chat")
                .required(false)
                .num_args(0) // accepts nothing
                .long_help("if true then returns new chat object!"),
        )
        .arg(
            Arg::new("get_all_chats")
                .long("get-all-chats")
                .required(false)
                .num_args(0) // accepts nothing
                .long_help("fetches all chat in db!"),
        )
        .arg(
            Arg::new("get_all_messages")
                .long("get-all-messages")
                .required(false)
                .value_name("ID i64")
                .num_args(1) //i64
                .long_help("requires to provide the chat id!"),
        )
        .arg(
            Arg::new("get_summary_of_chat")
                .long("get-summary-of-chat")
                .required(false)
                .value_name("ID i64")
                .num_args(1)
                .long_help("returns the summary context of a chat !"),
        )
        .arg(
            Arg::new("get_chat_by_id")
                .long("get-chat-by-id")
                .value_name("ID i64")
                .required(false)
                .num_args(1)
                .long_help("returns a chat by providing its id!"),
        )
        .get_matches();

    parse_args(app)
}

fn service_utils_command() -> Command {
    command!()
        .name("service_utils")
        .about("service utils")
        .arg(
            Arg::new("download_model_stream")
                .long("download-model-stream")
                .required(false)
                .value_name("NAME")
                .num_args(1)
                .long_help("accepts a model name : model_name:token_size!"),
        )
        .arg(
            Arg::new("download_model")
                .value_name("NAME")
                .long("download-model")
                .required(false)
                .num_args(1)
                .long_help("accepts a model_name:token_size to download to local storage"),
        )
        .arg(
            Arg::new("delete_model")
                .value_name("NAME")
                .long("delete-model")
                .required(false)
                .num_args(1)
                .long_help("accepts a model name to delete from the local storage"),
        )
        .arg(
            Arg::new("fetch_model_by_name")
                .value_name("NAME")
                .long("fetch-model-by-name")
                .required(false)
                .num_args(1)
                .long_help("fetches a model by its name not token size to provide!"),
        )
        .arg(
            Arg::new("install_tool")
                .long("install-tool")
                .value_name("PASSWORD")
                .required(false)
                .num_args(1)
                .long_help("installs a ollama tool, requires a sudo password!"),
        ) //--------------------------------------------------
        .arg(
            Arg::new("fetch_models_from_db")
                .long("fetch-models-from-db")
                .required(false)
                .num_args(0)
                .long_help("fetches all models information from the local database!"),
        )
        .arg(
            Arg::new("fetch_models_from_web_to_db")
                .long("fetch-models-from-web-to-db")
                .required(false)
                .num_args(0)
                .long_help("fetches models from the web and stores them in the database!"),
        )
        .arg(
            Arg::new("get_local_models_info")
                .long("get-local-models-info")
                .required(false)
                .num_args(0)
                .long_help("retrieves information about local downloaded models!"),
        )
        .arg(
            Arg::new("is_installed_globally")
                .long("is-installed-globally")
                .required(false)
                .num_args(0)
                .long_help("checks if a tool is installed globally"),
        )
        .arg(
            Arg::new("is_installed_locally")
                .long("is-installed-locally")
                .required(false)
                .num_args(0)
                .long_help("checks if a tool is installed locally"),
        )
        .arg(
            Arg::new("is_ollama_running")
                .long("is-ollama-running")
                .required(false)
                .num_args(0)
                .long_help("checks if the Ollama service is running"),
        )
        .arg(
            Arg::new("load_models_from_json_file")
                .long("load-models-from-json-file")
                .required(false)
                .num_args(0)
                .long_help("loads models from a JSON file"),
        )
        .arg(
            Arg::new("load_models_from_web_to_json")
                .long("load-models-from-web-to-json")
                .required(false)
                .num_args(0)
                .long_help("loads models from the web and stores them in a JSON file"),
        )
        .arg(
            Arg::new("start_ollama_service")
                .long("start-ollama-service")
                .required(false)
                .num_args(0)
                .long_help("starts the Ollama service "),
        )
}
fn create_message_command() -> Command {
    command!()
        .name("create_message")
        .arg(
            Arg::new("new_message")
                .long("message")
                .short('m')
                .value_name("text")
                .required(true)
                .num_args(1)
                .long_help("accepts a message from a user and returns a response!"),
        )
        .arg(
            Arg::new("chat_id")
                .long("chat-id")
                .required(true)
                .value_name("ID i64")
                .num_args(1)
                .long_help("chat id to assign the message!"),
        )
        .arg(
            Arg::new("model_name")
                .long("model-name")
                .value_name("Name")
                .required(true)
                .num_args(1)
                .long_help("the name of the model:token_size to chat with!"),
        )
}
