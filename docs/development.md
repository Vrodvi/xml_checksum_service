# Разработка

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
    └── tests.rs
```

## Зависимости

```toml
[package]
name = "xml_checksum_service"
version = "0.1.0"
edition = "2021"

[dependencies]
sha2 = "0.10"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
quick-xml = { version = "0.31", features = ["serialize"] }
encoding_rs = "0.8"
```

## Основные компоненты

1. **TCP-сервер** (`server.rs`)
    * Обработка входящих соединений
    * Многопоточность с помощью std::thread
    * Проверка состояния соединения
    * Отправка результатов клиентам

2. **Обработчик файлов** (`file_handler.rs`)
    * Поиск XML-файлов в директории
    * Фильтрация по расширению .xml
    * Парсинг XML и извлечение totalAmount
    * Удаление обработанных файлов

3. **Вычислитель хешей** (`hash_calculator.rs`)
    * Вычисление SHA-256
    * Потоковая обработка файлов
    * Форматирование хешей

## Формат XML-файлов

Сервер ожидает XML-файлы в следующем формате:

```xml
<?xml version="1.0"?>
<invoice>
  <invoiceNumber>INV-2025-001</invoiceNumber>
  <invoiceDate>2025-05-27</invoiceDate>
  <seller>
    <name>Acme Corp</name>
    <address>123 Main St, Anytown</address>
  </seller>
  <buyer>
    <name>Beta Inc</name>
    <address>456 Oak Ave, Anytown</address>
  </buyer>
  <lineItems>
    <lineItem>
      <description>Widget</description>
      <quantity>10</quantity>
      <unitPrice>20.00</unitPrice>
      <totalPrice>200.00</totalPrice>
    </lineItem>
  </lineItems>
  <subtotal>450.00</subtotal>
  <taxRate>10</taxRate>
  <taxAmount>45.00</taxAmount>
  <totalAmount>495.00</totalAmount>
</invoice>
```

## Результаты обработки

Для каждого XML-файла сервер возвращает:

1. SHA-256 хеш файла
2. Значение поля totalAmount

Формат результата:
```json
{
  "invoice.xml": "хеш_файла|сумма"
}
```

## Реализация

### 1. TCP-сервер

```rust
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

struct Server {
    listener: TcpListener,
    file_handler: Arc<Mutex<FileHandler>>,
}

impl Server {
    fn new(host: &str, port: u16, xml_directory: String) -> std::io::Result<Self> {
        let listener = TcpListener::bind(format!("{}:{}", host, port))?;
        let file_handler = Arc::new(Mutex::new(FileHandler::new(xml_directory)));
        Ok(Self { listener, file_handler })
    }

    fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let file_handler = Arc::clone(&self.file_handler);
                    std::thread::spawn(move || {
                        Self::handle_client(stream, file_handler);
                    });
                }
                Err(e) => eprintln!("Ошибка соединения: {}", e),
            }
        }
    }
}
```

### 2. Обработчик файлов

```rust
use std::path::Path;
use std::fs;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Invoice {
    #[serde(rename = "totalAmount")]
    total_amount: String,
}

struct FileHandler {
    directory: String,
    hash_calculator: HashCalculator,
}

impl FileHandler {
    fn new(directory: String) -> Self {
        Self { 
            directory,
            hash_calculator: HashCalculator::new(),
        }
    }

    fn process_files(&self) -> HashMap<String, String> {
        // Реализация обработки файлов
    }

    fn delete_processed_files(&self, file_names: &[String]) {
        // Реализация удаления файлов
    }
}
```

### 3. Вычислитель хешей

```rust
use sha2::{Sha256, Digest};

struct HashCalculator;

impl HashCalculator {
    fn new() -> Self {
        Self
    }

    fn calculate_hash(&self, file_path: &Path) -> Result<String, Error> {
        // Реализация вычисления хеша
    }
}
```

## Многопоточность

1. **std::thread**
    * Каждое соединение в отдельном потоке
    * Безопасный доступ к общим ресурсам через Arc<Mutex<>>
    * Контроль состояния соединения

2. **Обработка соединений**
    * Проверка активности соединения
    * Безопасное завершение при отключении клиента
    * Контроль ресурсов

## Обработка ошибок

1. **Типы ошибок**
    * `io::Error` - ошибки ввода/вывода
    * `HashError` - ошибки хеширования
    * `XmlError` - ошибки парсинга XML

2. **Обработка исключений**
    * Логирование ошибок
    * Проверка состояния соединения
    * Безопасное завершение операций

## Следующие шаги

1. [Тестирование](testing.md) - проверка работоспособности