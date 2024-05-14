#!/bin/bash

# функция для запуска команды alog
run_alog() {
	cargo run --bin alog -- "$@"
}

# обработка аргументов командной строки
if [ "$#" -eq 0 ]; then
	echo "usage: $0 [-h] [command]"
	echo "commands:"
	echo "  -h, --help    show this help message"
	echo "  [command]     run 'alog' with the specified command"
	exit 1
fi

case "$1" in
-h | --help)
	run_alog -h
	;;
*)
	run_alog "$@"
	;;
esac
