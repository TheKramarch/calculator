//! Арифметические операции с проверкой ошибок.

use crate::parser::CalcError;

/// Деление с проверкой делителя.
pub fn div(left: f64, right: f64) -> Result<f64, CalcError> {
    if right == 0.0 {
        Err(CalcError::DivisionByZero)
    } else {
        Ok(left / right)
    }
}

/// Остаток от деления с проверкой делителя.
pub fn rem(left: f64, right: f64) -> Result<f64, CalcError> {
    if right == 0.0 {
        Err(CalcError::ModuloByZero)
    } else {
        Ok(left % right)
    }
}

/// Возведение в степень.
pub fn pow(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// Квадратный корень с проверкой области определения.
pub fn sqrt(value: f64) -> Result<f64, CalcError> {
    if value < 0.0 {
        Err(CalcError::NegativeSquareRoot)
    } else {
        Ok(value.sqrt())
    }
}

/// Модуль числа.
pub fn abs(value: f64) -> f64 {
    value.abs()
}
