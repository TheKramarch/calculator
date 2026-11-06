//! Рекурсивный спуск: выражение -> слагаемое -> унарный минус -> степень -> атом.

use std::fmt;

use crate::ops;

/// Ошибка разбора или вычисления выражения.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalcError {
    EmptyExpression,
    DivisionByZero,
    ModuloByZero,
    InvalidNumber(String),
    UnexpectedToken(char),
    UnexpectedEnd,
    UnclosedParenthesis,
    ExpectedParenthesis(String),
    UnknownFunction(String),
    NegativeSquareRoot,
    NonPositiveLogarithm,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::EmptyExpression => write!(f, "выражение пустое"),
            CalcError::DivisionByZero => write!(f, "деление на ноль"),
            CalcError::ModuloByZero => write!(f, "остаток от деления на ноль"),
            CalcError::InvalidNumber(raw) => write!(f, "некорректное число: {raw}"),
            CalcError::UnexpectedToken(c) => write!(f, "неожиданный символ: '{c}'"),
            CalcError::UnexpectedEnd => write!(f, "неожиданный конец выражения"),
            CalcError::UnclosedParenthesis => write!(f, "не закрыта скобка"),
            CalcError::ExpectedParenthesis(name) => {
                write!(f, "после функции {name} ожидалась '('")
            }
            CalcError::UnknownFunction(name) => write!(f, "неизвестная функция: {name}"),
            CalcError::NegativeSquareRoot => {
                write!(f, "квадратный корень из отрицательного числа")
            }
            CalcError::NonPositiveLogarithm => {
                write!(f, "логарифм неположительного числа")
            }
        }
    }
}

impl std::error::Error for CalcError {}

/// Вычисляет арифметическое выражение.
///
/// Поддерживаются `+`, `-`, `*`, `/`, `%`, `^`, скобки, унарный минус,
/// а также функции `sqrt`, `abs`, `round`, `floor`, `ceil`, `ln` и `log10`.
pub fn evaluate(input: &str) -> Result<f64, CalcError> {
    let mut parser = Parser::new(input);
    parser.skip_whitespace();
    if parser.peek().is_none() {
        return Err(CalcError::EmptyExpression);
    }

    let value = parser.parse_expression()?;
    parser.skip_whitespace();
    if let Some(c) = parser.peek() {
        return Err(CalcError::UnexpectedToken(c));
    }

    Ok(value)
}

struct Parser {
    chars: Vec<char>,
    pos: usize,
}

impl Parser {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let current = self.peek();
        if current.is_some() {
            self.pos += 1;
        }
        current
    }

    fn skip_whitespace(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.pos += 1;
        }
    }

    fn parse_expression(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_term()?;
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('+') => {
                    self.advance();
                    value += self.parse_term()?;
                }
                Some('-') => {
                    self.advance();
                    value -= self.parse_term()?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_term(&mut self) -> Result<f64, CalcError> {
        let mut value = self.parse_unary()?;
        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('*') => {
                    self.advance();
                    value *= self.parse_unary()?;
                }
                Some('/') => {
                    self.advance();
                    let right = self.parse_unary()?;
                    value = ops::div(value, right)?;
                }
                Some('%') => {
                    self.advance();
                    let right = self.parse_unary()?;
                    value = ops::rem(value, right)?;
                }
                _ => return Ok(value),
            }
        }
    }

    fn parse_unary(&mut self) -> Result<f64, CalcError> {
        self.skip_whitespace();
        match self.peek() {
            Some('-') => {
                self.advance();
                Ok(-self.parse_unary()?)
            }
            Some('+') => {
                self.advance();
                self.parse_unary()
            }
            _ => self.parse_power(),
        }
    }

    fn parse_power(&mut self) -> Result<f64, CalcError> {
        let base = self.parse_primary()?;
        self.skip_whitespace();
        if self.peek() == Some('^') {
            self.advance();
            // Правая ассоциативность: 2^3^2 == 2^(3^2).
            let exponent = self.parse_unary()?;
            Ok(ops::pow(base, exponent))
        } else {
            Ok(base)
        }
    }

    fn parse_primary(&mut self) -> Result<f64, CalcError> {
        self.skip_whitespace();
        match self.peek() {
            Some('(') => {
                self.advance();
                let value = self.parse_expression()?;
                self.skip_whitespace();
                if self.advance() != Some(')') {
                    return Err(CalcError::UnclosedParenthesis);
                }
                Ok(value)
            }
            Some(c) if c.is_ascii_digit() || c == '.' => self.parse_number(),
            Some(c) if c.is_alphabetic() => self.parse_function(),
            Some(c) => Err(CalcError::UnexpectedToken(c)),
            None => Err(CalcError::UnexpectedEnd),
        }
    }

    fn parse_number(&mut self) -> Result<f64, CalcError> {
        let start = self.pos;
        let mut seen_dot = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !seen_dot {
                seen_dot = true;
                self.advance();
            } else {
                break;
            }
        }

        let raw: String = self.chars[start..self.pos].iter().collect();
        raw.parse::<f64>()
            .map_err(|_| CalcError::InvalidNumber(raw))
    }

    fn parse_function(&mut self) -> Result<f64, CalcError> {
        let start = self.pos;
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
            self.advance();
        }
        let name: String = self.chars[start..self.pos].iter().collect();

        self.skip_whitespace();
        if self.advance() != Some('(') {
            return Err(CalcError::ExpectedParenthesis(name));
        }

        let argument = self.parse_expression()?;
        self.skip_whitespace();
        if self.advance() != Some(')') {
            return Err(CalcError::UnclosedParenthesis);
        }

        match name.to_ascii_lowercase().as_str() {
            "sqrt" => ops::sqrt(argument),
            "abs" => Ok(ops::abs(argument)),
            "round" => Ok(argument.round()),
            "floor" => Ok(argument.floor()),
            "ceil" => Ok(argument.ceil()),
            "ln" => ops::ln(argument),
            "log10" => ops::log10(argument),
            _ => Err(CalcError::UnknownFunction(name)),
        }
    }
}
