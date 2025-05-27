# XML Checksum Service

Сервис для обработки XML-файлов, вычисления их хешей и извлечения суммы из структуры документа.

## Функциональность

- Мониторинг директории на наличие XML-файлов
- Вычисление SHA-256 хеша для каждого файла
- Извлечение суммы из атрибута `totalAmount`
- Отправка результатов в формате JSON
- Удаление обработанных файлов
- Поддержка кодировки Windows-1251

## Требования

- Rust 1.70 или выше
- Cargo

## Установка

1. Клонируйте репозиторий:
```bash
git clone https://github.com/Vrodvi/xml_checksum_service.git
cd xml_checksum_service
```

2. Соберите проект:
```bash
cargo build --release
```

## Использование

1. Создайте файл конфигурации `config.toml`:
```toml
[server]
host = "127.0.0.1"
port = 12345

[files]
xml_directory = "xml_files"
```

2. Запустите сервер:
```bash
cargo run --release
```

3. Подключитесь к серверу через telnet:
```bash
telnet localhost 12345
```

## Тестирование

Запуск тестов:
```bash
cargo test
```

## Документация

Подробная документация доступна в директории `docs/`:
- [Обзор проекта](docs/index.md)
- [Разработка](docs/development.md)
- [Тестирование](docs/testing.md)

## Лицензия

MIT 