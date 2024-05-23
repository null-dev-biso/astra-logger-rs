#### cargo build - установка зависимостей

```sh
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
