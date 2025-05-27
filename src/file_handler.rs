use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::hash_calculator::HashCalculator;
use quick_xml::de::from_str;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Invoice {
    #[serde(rename = "totalAmount")]
    total_amount: String,
}

pub struct FileHandler {
    directory: String,
    hash_calculator: HashCalculator,
}

impl FileHandler {
    pub fn new(directory: String) -> Self {
        println!("Создание обработчика файлов для директории: {}", directory);
        Self {
            directory,
            hash_calculator: HashCalculator::new(),
        }
    }

    pub fn process_files(&self) -> HashMap<String, String> {
        println!("Начало обработки файлов в директории: {}", self.directory);
        let mut results = HashMap::new();
        let path = Path::new(&self.directory);
        
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        let file_path = entry.path();
                        if let Some(ext) = file_path.extension() {
                            if ext == "xml" {
                                println!("Найден XML файл: {}", file_path.display());
                                
                                // Читаем содержимое файла
                                if let Ok(content) = fs::read_to_string(&file_path) {
                                    // Парсим XML
                                    if let Ok(invoice) = from_str::<Invoice>(&content) {
                                        // Вычисляем хеш
                                        if let Ok(hash) = self.hash_calculator.calculate_hash(&file_path) {
                                            let file_name = file_path.file_name()
                                                .unwrap()
                                                .to_string_lossy()
                                                .into_owned();
                                            
                                            println!("Вычислен хеш для файла: {}", file_name);
                                            println!("Сумма в файле: {}", invoice.total_amount);
                                            
                                            // Сохраняем хеш и сумму
                                            results.insert(file_name, format!("{}|{}", hash, invoice.total_amount));
                                        }
                                    } else {
                                        println!("Ошибка парсинга XML в файле: {}", file_path.display());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        println!("Обработка файлов завершена. Обработано файлов: {}", results.len());
        results
    }

    pub fn delete_processed_files(&self, file_names: &[String]) {
        println!("Начало удаления обработанных файлов");
        let path = Path::new(&self.directory);
        
        for file_name in file_names {
            let file_path = path.join(file_name);
            if file_path.exists() {
                println!("Удаление файла: {}", file_name);
                let _ = fs::remove_file(file_path);
            }
        }
        println!("Удаление файлов завершено");
    }
} 