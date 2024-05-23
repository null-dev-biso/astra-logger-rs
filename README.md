#### Скачайте Rust перед запуском!
###### Основные моменты
```sh Команда помощи 
./alog.sh -h
```

```sh
Usage: alog [OPTIONS]

Options:
  -p, --paths <PATHS>              Путь к файлу или директории с логами
  -l, --pattern <PATTERN>          Регулярное выражение для фильтрации строк логов [default: ]
  -s, --system-info                Вывод базовой информации о системе
  -j, --output-json <OUTPUT_JSON>  Сохранение файла для удобного формата логов в json
  -h, --help                       Print help
  -V, --version                    Print version

```

```sh
ПРИМЕР
./alog.sh -p ./log/daemon.log -l " " -j ./log/daemon.json --tui
```

###### Тесты 
Тесты находятся в директории src/test.rs 
```sh Запуск тестов
cargo test
```

| Тесты     | Описание                             | Аппрув |
| --------- | ------------------------------------ | ------ |
| Юнит      | test_log_entry_new                   | +      |
|           | test_logs_new                        | +      |
|           | test_log_stats_new                   | +      |
|           | test_elliptic_curve_calculate_points | +      |
| Системные | test_analyze_log_line                | +      |
|           | test_format_to_json                  | +      |
|           | test_log_stats_analyze_log_line      | +      |

