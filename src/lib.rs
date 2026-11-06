//! Библиотека калькулятора: разбор и вычисление арифметических выражений.

pub mod ops;
pub mod parser;

pub use parser::{evaluate, CalcError};

/// Форматирует результат для вывода: целые числа печатаются без `.0`.
pub fn format_value(value: f64) -> String {
    if value.is_nan() {
        "NaN".to_string()
    } else if value.is_infinite() {
        if value.is_sign_positive() {
            "∞".to_string()
        } else {
            "-∞".to_string()
        }
    } else if value == 0.0 {
        "0".to_string()
    } else if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{value:.0}")
    } else {
        format!("{value}")
    }
}
