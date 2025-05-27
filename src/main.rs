mod server;
mod file_handler;
mod hash_calculator;

use crate::server::Server;

use std::fs;
use std::path::Path;
use std::env;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
struct Config {
    server: ServerConfig,
    files: FilesConfig,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    port: u16,
    host: String,
}

#[derive(Debug, Deserialize)]
struct FilesConfig {
    xml_directory: String,
}

fn find_config_file() -> Option<String> {
    // 1. Рядом с исполняемым файлом
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let config_path = exe_dir.join("config.toml");
            if config_path.exists() {
                return Some(config_path.to_string_lossy().into_owned());
            }
        }
    }

    // 2. В текущей рабочей директории
    let current_dir = env::current_dir().ok()?;
    let config_path = current_dir.join("config.toml");
    if config_path.exists() {
        return Some(config_path.to_string_lossy().into_owned());
    }

    // 3. В директории проекта (для разработки)
    let project_config = "config.toml";
    if Path::new(project_config).exists() {
        return Some(project_config.to_string());
    }

    None
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = find_config_file()
        .ok_or_else(|| "Не найден файл конфигурации config.toml".to_string())?;
    
    println!("Используется конфигурационный файл: {}", config_path);
    
    let config_str = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;
    
    // Создаем директорию, если она не существует
    let xml_dir = Path::new(&config.files.xml_directory);
    if !xml_dir.exists() {
        fs::create_dir_all(xml_dir)?;
    }
    
    println!("Мониторинг папки: {}", config.files.xml_directory);
    
    let server = Server::new(
        &config.server.host,
        config.server.port,
        config.files.xml_directory.clone()
    )?;
    
    server.run();
    
    Ok(())
}
