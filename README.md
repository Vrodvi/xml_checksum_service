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
- Python 3.7+ (для документации)
- pip (для установки MkDocs)

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

3. Установите MkDocs и необходимые плагины:
```bash
pip install mkdocs mkdocs-material
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

### Локальный запуск документации

1. Перейдите в директорию проекта:
```bash
cd xml_checksum_service
```

2. Запустите локальный сервер документации:
```bash
mkdocs serve
```

3. Откройте в браузере: http://127.0.0.1:8000

### Сборка документации

Для сборки статической версии документации:
```bash
mkdocs build
```

Собранная документация будет доступна в директории `site/`.

### Публикация документации

1. Соберите документацию:
```bash
mkdocs build
```

2. Опубликуйте на GitHub Pages:
```bash
mkdocs gh-deploy
```

Документация будет доступна по адресу: https://vrodvi.github.io/xml_checksum_service/

Подробная документация доступна в директории `docs/`:
- [Обзор проекта](docs/index.md)
- [Разработка](docs/development.md)
- [Тестирование](docs/testing.md)

## Лицензия

MIT 