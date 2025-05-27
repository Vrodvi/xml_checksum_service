use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::Write;
use encoding_rs::WINDOWS_1251;
use crate::file_handler::FileHandler;

pub struct Server {
    listener: TcpListener,
    file_handler: Arc<Mutex<FileHandler>>,
}

impl Server {
    pub fn new(host: &str, port: u16, xml_directory: String) -> std::io::Result<Self> {
        println!("Инициализация сервера...");
        let listener = TcpListener::bind(format!("{}:{}", host, port))?;
        let file_handler = Arc::new(Mutex::new(FileHandler::new(xml_directory)));
        
        Ok(Self {
            listener,
            file_handler,
        })
    }

    pub fn run(&self) {
        println!("Сервер запущен на {}", self.listener.local_addr().unwrap());
        println!("Ожидание подключений...");
        
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Получено новое подключение");
                    let file_handler = Arc::clone(&self.file_handler);
                    std::thread::spawn(move || {
                        Self::handle_client(stream, file_handler);
                    });
                }
                Err(e) => {
                    eprintln!("Ошибка соединения: {}", e);
                }
            }
        }
    }

    fn handle_client(stream: TcpStream, file_handler: Arc<Mutex<FileHandler>>) {
        // Получаем информацию о клиенте
        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(e) => {
                println!("Ошибка получения адреса клиента: {}", e);
                return;
            }
        };
        println!("Подключение от клиента: {}", peer_addr);
        
        let mut writer = stream;
        
        // Блокируем обработчик файлов на время всей операции
        let handler = file_handler.lock().unwrap();
        
        println!("Обработка файлов...");
        let results = handler.process_files();
        
        // Формируем JSON
        let json_response = serde_json::to_string_pretty(&results).unwrap();
        
        // Конвертируем в Windows-1251
        let (encoded, _, had_errors) = WINDOWS_1251.encode(&json_response);
        if had_errors {
            println!("Предупреждение: были ошибки при конвертации в Windows-1251");
        }
        
        // Проверяем, что соединение все еще активно
        if writer.write_all(&encoded).is_err() {
            println!("Ошибка отправки результатов клиенту {}", peer_addr);
            return;
        }
        
        if writer.flush().is_err() {
            println!("Ошибка отправки результатов клиенту {}", peer_addr);
            return;
        }
        
        println!("Результаты успешно отправлены клиенту {}", peer_addr);
        
        // Удаляем файлы только если успешно отправили результаты
        handler.delete_processed_files(&results.keys().cloned().collect::<Vec<_>>());
    }
} 