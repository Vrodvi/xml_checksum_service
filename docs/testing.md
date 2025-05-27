# Тестирование

## Обзор тестов

Проект включает в себя интеграционные тесты, которые проверяют основную функциональность сервиса. Тесты находятся в файле `tests/tests.rs`.

## Структура тестов

### Тест подключения к серверу
```rust
#[test]
fn test_server_connection() {
    let server = Server::new("127.0.0.1", 12345, "test_xml_files_server".to_string()).unwrap();
    std::thread::spawn(move || {
        server.run();
    });
    
    // Даем серверу время на запуск
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    let result = TcpStream::connect("127.0.0.1:12345");
    assert!(result.is_ok());
}
```
Этот тест проверяет:
- Создание сервера
- Запуск сервера в отдельном потоке
- Возможность подключения к серверу

### Тест обработки файлов
```rust
#[test]
fn test_file_processing() {
    let test_dir = "test_xml_files_processing";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();
    
    // Создание тестового XML-файла
    let test_file = Path::new(test_dir).join("invoice.xml");
    let xml_content = r#"
        <?xml version="1.0"?>
        <invoice>
            <totalAmount>495.00</totalAmount>
        </invoice>
    "#;
    fs::write(&test_file, xml_content).unwrap();
    
    let handler = FileHandler::new(test_dir.to_string());
    let results = handler.process_files();
    
    assert!(results.contains_key("invoice.xml"));
    let result = results.get("invoice.xml").unwrap();
    assert!(result.contains("|495.00"));
    
    handler.delete_processed_files(&results.keys().cloned().collect::<Vec<_>>());
    assert!(!test_file.exists());
    
    fs::remove_dir_all(test_dir).unwrap();
}
```
Этот тест проверяет:
- Создание и очистку тестовой директории
- Обработку XML-файла
- Извлечение суммы из XML
- Удаление обработанного файла

### Тест вычисления хеша
```rust
#[test]
fn test_hash_calculation() {
    let calculator = HashCalculator::new();
    
    let test_file = "test.xml";
    let content = "<test>Hello, World!</test>";
    fs::write(test_file, content).unwrap();
    
    let result = calculator.calculate_hash(Path::new(test_file));
    assert!(result.is_ok());
    
    let hash = result.unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 64);
    
    fs::remove_file(test_file).unwrap();
}
```
Этот тест проверяет:
- Создание калькулятора хешей
- Вычисление хеша файла
- Корректность длины хеша (64 символа для SHA-256)

### Тест парсинга XML
```rust
#[test]
fn test_xml_parsing() {
    let xml_content = r#"
        <?xml version="1.0"?>
        <invoice>
            <totalAmount>495.00</totalAmount>
        </invoice>
    "#;
    
    let invoice: Invoice = from_str(xml_content).unwrap();
    assert_eq!(invoice.total_amount, "495.00");
}
```
Этот тест проверяет:
- Корректность парсинга XML
- Извлечение значения totalAmount

### Тест обработки некорректного XML
```rust
#[test]
fn test_invalid_xml() {
    let xml_content = r#"
        <?xml version="1.0"?>
        <invoice>
            <invalidTag>495.00</invalidTag>
        </invoice>
    "#;
    
    let result: Result<Invoice, _> = from_str(xml_content);
    assert!(result.is_err());
}
```
Этот тест проверяет:
- Обработку некорректного XML
- Корректность сообщения об ошибке

### Тест обработки нескольких файлов
```rust
#[test]
fn test_multiple_files() {
    let test_dir = "test_xml_files_multiple";
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();
    
    let files = vec![
        ("invoice1.xml", "495.00"),
        ("invoice2.xml", "1234.56"),
    ];
    
    for (filename, amount) in &files {
        let file_path = Path::new(test_dir).join(filename);
        let xml_content = format!(
            r#"<?xml version="1.0"?>
            <invoice>
                <totalAmount>{}</totalAmount>
            </invoice>"#,
            amount
        );
        fs::write(&file_path, xml_content).unwrap();
    }
    
    let handler = FileHandler::new(test_dir.to_string());
    let results = handler.process_files();
    
    assert_eq!(results.len(), 2);
    assert!(results.contains_key("invoice1.xml"));
    assert!(results.contains_key("invoice2.xml"));
    
    assert!(results.get("invoice1.xml").unwrap().contains("|495.00"));
    assert!(results.get("invoice2.xml").unwrap().contains("|1234.56"));
    
    handler.delete_processed_files(&results.keys().cloned().collect::<Vec<_>>());
    
    for (filename, _) in &files {
        assert!(!Path::new(test_dir).join(filename).exists());
    }
    
    fs::remove_dir_all(test_dir).unwrap();
}
```
Этот тест проверяет:
- Обработку нескольких XML-файлов
- Корректность извлечения сумм из разных файлов
- Удаление всех обработанных файлов

## Запуск тестов

Для запуска тестов используйте команду:
```bash
cargo test
```

Для запуска конкретного теста:
```bash
cargo test test_name
```

## Отладка тестов

Для получения более подробной информации о выполнении тестов используйте:
```bash
cargo test -- --nocapture
```

Для запуска тестов с выводом стека вызовов при ошибке:
```bash
cargo test -- --nocapture --test-threads=1
```

## Примечания по тестированию

1. Каждый тест использует свою уникальную тестовую директорию для избежания конфликтов
2. Все тестовые файлы и директории удаляются после выполнения тестов
3. Тесты проверяют как успешные сценарии, так и обработку ошибок
4. Для тестов сервера используется отдельный порт (12345) 