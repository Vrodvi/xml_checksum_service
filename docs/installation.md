# Установка

## Предварительные требования

* Rust 1.70.0 или выше
* Cargo (менеджер пакетов Rust)
* Git (для управления версиями)

## Установка Rust

1. **Windows**
    * Скачайте установщик с [официального сайта](https://www.rust-lang.org/tools/install)
    * Запустите `rustup-init.exe`
    * Следуйте инструкциям установщика

2. **Linux/macOS**
    * Выполните команду:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
    * Перезапустите терминал

## Проверка установки

1. Проверьте версию Rust:
```bash
rustc --version
```

2. Проверьте версию Cargo:
```bash
cargo --version
```

## Создание проекта

1. Создайте новый проект:
```bash
cargo new xml_checksum_service
cd xml_checksum_service
```

2. Настройте зависимости в `Cargo.toml`:
```toml
[package]
name = "xml_checksum_service"
version = "0.1.0"
edition = "2021"

[dependencies]
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
toml = "0.8"
```

## Настройка конфигурации

1. Создайте файл `config.toml` в корневой директории проекта:
```toml
[server]
port = 8080
host = "127.0.0.1"

[files]
xml_directory = "C:/xml_files"  # для Windows
# xml_directory = "~/xml_files"  # для Linux/macOS
```

2. Настройте параметры:
    * `port` - порт для TCP-соединений
    * `host` - IP-адрес сервера
    * `xml_directory` - путь к директории с XML-файлами

## Структура проекта

```
xml_checksum_service/
├── Cargo.toml
├── config.toml
├── src/
│   ├── main.rs
│   ├── server.rs
│   ├── file_handler.rs
│   └── hash_calculator.rs
└── tests/
    └── integration_tests.rs
```

## Сборка проекта

```bash
cargo build --release
```

## Запуск сервера

```bash
cargo run --release
```

## Проверка работоспособности

1. Создайте тестовый XML-файл:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<test>
    <data>Hello, World!</data>
</test>
```

2. Поместите файл в директорию, указанную в `config.toml`

3. Подключитесь к серверу:

```bash
telnet localhost 8080
```

## Возможные проблемы

### Порт занят

Если порт 8080 занят, измените его в `config.toml`:
```toml
[server]
port = 8081  # Измените на свободный порт
```

### Проблемы с правами доступа

Убедитесь, что у вас есть права на:
* Чтение директории с XML-файлами
* Удаление файлов
* Открытие порта для TCP-соединений

## Следующие шаги

1. [Разработка](development.md) - создание сервиса
2. [Тестирование](testing.md) - проверка работоспособности 