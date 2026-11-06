# calculator

[![CI](https://github.com/TheKramarch/calculator/actions/workflows/ci.yml/badge.svg)](https://github.com/TheKramarch/calculator/actions/workflows/ci.yml)

Простой калькулятор на Rust с двумя режимами работы: вычисление выражения из
аргументов командной строки и интерактивный REPL.

## Возможности

- Операции `+`, `-`, `*`, `/`, `%`, `^`
- Скобки и унарный минус
- Функции `sqrt`, `abs`, `round`, `floor`, `ceil`
- Понятные сообщения об ошибках (деление на ноль, мусорный ввод и т. д.)
- Без внешних зависимостей

## Сборка и запуск

```bash
cargo build --release
cargo run -- "2 + 3 * 4"   # 14
cargo run                  # интерактивный режим
```

Пример интерактивной сессии:

```
Калькулятор. Введите выражение (exit — выход).
> sqrt(16) + 2^3
12
> 1 / 0
Ошибка: деление на ноль
> exit
```

## Тесты

```bash
cargo test
```

## Структура проекта

```
src/lib.rs      — публичное API и форматирование результата
src/parser.rs   — рекурсивный спуск и тип ошибки
src/ops.rs      — арифметические операции с проверками
src/main.rs     — CLI и REPL
tests/calc.rs   — интеграционные тесты
```

## Лицензия

MIT
