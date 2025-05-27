use std::fs;
use std::path::Path;
use std::net::TcpStream;
use xml_checksum_service::server::Server;
use xml_checksum_service::file_handler::FileHandler;
use xml_checksum_service::hash_calculator::HashCalculator;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Invoice {
    #[serde(rename = "totalAmount")]
    total_amount: String,
}

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

#[test]
fn test_file_processing() {
    // Создаем тестовую директорию с уникальным именем
    let test_dir = "test_xml_files_processing";
    // Очищаем директорию перед тестом
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();
    
    // Создаем тестовый XML-файл с корректной структурой
    let test_file = Path::new(test_dir).join("invoice.xml");
    let xml_content = r#"
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
    "#;
    fs::write(&test_file, xml_content).unwrap();
    
    let handler = FileHandler::new(test_dir.to_string());
    let results = handler.process_files();
    
    assert!(results.contains_key("invoice.xml"));
    let result = results.get("invoice.xml").unwrap();
    assert!(result.contains("|495.00")); // Проверяем наличие суммы в результате
    
    // Явно удаляем обработанные файлы
    handler.delete_processed_files(&results.keys().cloned().collect::<Vec<_>>());
    
    assert!(!test_file.exists()); // Файл должен быть удален
    
    // Очищаем тестовую директорию
    fs::remove_dir_all(test_dir).unwrap();
}

#[test]
fn test_hash_calculation() {
    let calculator = HashCalculator::new();
    
    // Создаем тестовый файл
    let test_file = "test.xml";
    let content = "<test>Hello, World!</test>";
    fs::write(test_file, content).unwrap();
    
    let result = calculator.calculate_hash(Path::new(test_file));
    assert!(result.is_ok());
    
    // Проверяем, что хеш не пустой
    let hash = result.unwrap();
    assert!(!hash.is_empty());
    assert_eq!(hash.len(), 64); // SHA-256 хеш должен быть 64 символа
    
    // Очищаем тестовый файл
    fs::remove_file(test_file).unwrap();
}

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

#[test]
fn test_multiple_files() {
    // Создаем тестовую директорию с уникальным именем
    let test_dir = "test_xml_files_multiple";
    // Очищаем директорию перед тестом
    let _ = fs::remove_dir_all(test_dir);
    fs::create_dir_all(test_dir).unwrap();
    
    // Создаем несколько тестовых XML-файлов
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
    
    // Проверяем, что все файлы обработаны
    assert_eq!(results.len(), 2);
    assert!(results.contains_key("invoice1.xml"));
    assert!(results.contains_key("invoice2.xml"));
    
    // Проверяем суммы в результатах
    assert!(results.get("invoice1.xml").unwrap().contains("|495.00"));
    assert!(results.get("invoice2.xml").unwrap().contains("|1234.56"));
    
    // Явно удаляем обработанные файлы
    handler.delete_processed_files(&results.keys().cloned().collect::<Vec<_>>());
    
    // Проверяем, что файлы удалены
    for (filename, _) in &files {
        assert!(!Path::new(test_dir).join(filename).exists());
    }
    
    // Очищаем тестовую директорию
    fs::remove_dir_all(test_dir).unwrap();
} 