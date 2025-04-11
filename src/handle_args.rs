use crate::Config;
use anyhow::{Result, anyhow};
use clap::ArgMatches;
use ollama_rs::models::pull::PullModelStatusStream;
use omama_manager::{
    chat::{OConfig, StreamingResult, create_chat, create_message},
    database::{get_all_chats, get_all_messages, get_chat_by_id, get_summary_of_chat},
    service_utils::{
        delete_model, download_model, download_model_stream, fetch_model_by_name,
        fetch_models_from_db, fetch_models_from_web_to_db, get_local_models_info, install_tool,
        is_installed_globally, is_installed_locally, is_ollama_running, load_models_from_json_file,
        load_models_from_web_to_json, start_ollama_service,
    },
};
use serde::Serialize;
use serde_json::to_string_pretty;
use std::io::{Stdout, Write, stdout};
use tokio_stream::StreamExt;

pub async fn run(config: Config) -> Result<()> {
    run_process(config).await?;
    Ok(())
}

fn write_to_stdout<T: Serialize>(out: &mut Stdout, output: T) -> Result<()> {
    out.write_all(to_string_pretty(&output)?.as_bytes())?;
    out.flush()?;
    Ok(())
}
async fn run_process(config: Config) -> Result<()> {
    let mut out = stdout();
    //------------------bolean process!-------------
    if config.create_chat {
        let chat = create_chat().await;
        return write_to_stdout(&mut out, chat);
    }
    if config.fetch_models_from_db {
        let models_db = fetch_models_from_db().await?;
        return write_to_stdout(&mut out, &models_db);
    }
    if config.fetch_models_from_web_to_db {
        return fetch_models_from_web_to_db().await;
    }
    if config.load_models_from_web_to_json {
        return write_to_stdout(&mut out, load_models_from_web_to_json().await?);
    }
    if config.load_models_from_json_file {
        return write_to_stdout(&mut out, load_models_from_json_file().await?);
    }
    if config.get_all_chats {
        let chats = get_all_chats().await?;
        return write_to_stdout(&mut out, &chats);
    }
    if config.get_local_models_info {
        return write_to_stdout(&mut out, get_local_models_info().await?);
    }
    if config.is_ollama_running {
        return write_to_stdout(&mut out, is_ollama_running().await);
    }
    if config.is_installed_locally {
        return write_to_stdout(&mut out, is_installed_locally()?);
    }
    if config.is_installed_globally {
        return write_to_stdout(&mut out, is_installed_globally());
    }
    if config.start_ollama_service {
        // maybe differs from windows and mac!!
        return start_ollama_service(is_installed_globally()).await;
    }
    //---------------------------------arguments processes!---------------
    if let Some(oconf) = config.create_message {
        let message = create_message(oconf, stream_chat).await?;
        return write_to_stdout(&mut out, message);
    }
    if let Some(c_id) = config.get_summary_of_chat {
        let summary = get_summary_of_chat(c_id).await?;
        write_to_stdout(&mut out, summary)?;
        return Ok(());
    }
    if let Some(c_id) = config.get_all_messages {
        let messages = get_all_messages(c_id).await?;
        write_to_stdout(&mut out, messages)?;
        return Ok(());
    }
    if let Some(model_name) = config.fetch_model_by_name {
        let model = fetch_model_by_name(&model_name).await?;
        write_to_stdout(&mut out, model)?;
        return Ok(());
    }
    if let Some(full_name) = config.delete_model {
        let (name, token_size) = full_name
            .split_once(':')
            .ok_or_else(|| anyhow!("Invalid model name"))?;
        delete_model(name, token_size).await?;
        return Ok(());
    }
    if let Some(model_name) = config.download_model {
        let (name, token_size) = model_name
            .split_once(':')
            .ok_or_else(|| anyhow!("Invalid model name"))?;
        download_model(name, token_size).await?;
        return Ok(());
    }
    if let Some(c_id) = config.get_chat_by_id {
        let chat = get_chat_by_id(c_id).await?;
        write_to_stdout(&mut out, chat)?;
        return Ok(());
    }
    if let Some(model_name) = config.download_model_stream {
        let (name, token_size) = model_name
            .split_once(':')
            .ok_or_else(|| anyhow!("Invalid model name"))?;
        download_model_stream(name, token_size, stream_download_helper).await?;
        return Ok(());
    }
    if let Some(pass) = config.install_tool {
        #[cfg(target_os = "linux")]
        install_tool(&pass).await?;

        return Ok(());
    }

    Ok(())
}
async fn stream_download_helper(mut status: PullModelStatusStream) -> Result<()> {
    while let Some(s) = status.next().await {
        let ms = s?;
        println!(
            "\r{:.2}",
            (ms.completed.unwrap_or(0) as f64 / ms.total.unwrap_or(1) as f64) * 100.0
        );
        stdout().flush()?;
    }
    Ok(())
}
async fn stream_chat(mut streamer: StreamingResult) -> String {
    let mut response = "".to_owned();
    while let Some(Ok(word)) = streamer.next().await {
        response.push_str(&word.to_string());
    }

    response
}

pub(super) fn parse_args(args: ArgMatches) -> Result<Config> {
    let mut config = Config::default();

    let create_m_command = args.subcommand_matches("create_message");
    let service_u_command = args.subcommand_matches("service_utils");
    if let Some(cmc) = create_m_command {
        let message = cmc.get_one::<String>("new_message").unwrap();
        let chat_id = cmc
            .get_one::<String>("chat_id")
            .map(|t| t.trim().parse::<i64>().unwrap())
            .unwrap();
        let m_name = cmc.get_one::<String>("model_name").unwrap();
        config.set_create_message(Some(OConfig {
            user_message: message.to_owned(),
            c_id: chat_id,
            model_name: m_name.to_owned(),
        }));
    }
    if let Some(suc) = service_u_command {
        config.set_delete_model(suc.get_one::<String>("delete_model").map(|v| v.to_owned()));
        config.set_download_model_stream(
            suc.get_one::<String>("download_model_stream")
                .map(|v| v.to_owned()),
        );
        config.set_download_model(
            suc.get_one::<String>("download_model")
                .map(|v| v.to_owned()),
        );
        config.set_fetch_model_by_name(
            suc.get_one::<String>("fetch_model_by_name")
                .map(|v| v.to_owned()),
        );
        config.set_install_tool(suc.get_one::<String>("install_tool").map(|v| v.to_owned()));

        config.set_fetch_models_from_db(suc.get_flag("fetch_models_from_db"));
        config.set_fetch_models_from_web_to_db(suc.get_flag("fetch_models_from_web_to_db"));
        config.set_get_local_models_info(suc.get_flag("get_local_models_info"));
        config.set_is_installed_globally(suc.get_flag("is_installed_globally"));
        config.set_is_installed_locally(suc.get_flag("is_installed_locally"));
        config.set_is_ollama_running(suc.get_flag("is_ollama_running"));
        config.set_load_models_from_json_file(suc.get_flag("load_models_from_json_file"));
        config.set_load_models_from_web_to_json(suc.get_flag("load_models_from_web_to_json"));
        config.set_start_ollama_service(suc.get_flag("start_ollama_service"));
    }
    config.set_create_chat(args.get_flag("create_chat"));
    config.set_get_all_chats(args.get_flag("get_all_chats"));
    config.set_get_all_messages(
        args.get_one::<String>("get_all_messages")
            .map(|t| t.trim().parse::<i64>().unwrap()),
    );
    config.set_get_summary_of_chat(
        args.get_one::<String>("get_summary_of_chat")
            .map(|t| t.trim().parse::<i64>().unwrap()),
    );
    config.set_get_chat_by_id(
        args.get_one::<String>("get_chat_by_id")
            .map(|t| t.trim().parse::<i64>().unwrap()),
    );

    Ok(config)
}

#[cfg(test)]
mod quick_test {
    use omama_manager::service_utils::is_installed_globally;

    #[tokio::test]
    async fn test_installed_globally() {
        let service = is_installed_globally();
        dbg!(service);
    }
}
