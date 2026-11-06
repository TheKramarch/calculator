use calculator::{evaluate, format_value, CalcError};

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1e-10,
        "ожидалось {expected}, получено {actual}"
    );
}

#[test]
fn respects_operator_precedence() {
    assert_close(evaluate("2 + 3 * 4").unwrap(), 14.0);
}

#[test]
fn respects_parentheses() {
    assert_close(evaluate("(2 + 3) * 4").unwrap(), 20.0);
}

#[test]
fn divides_fractions() {
    assert_close(evaluate("10 / 4").unwrap(), 2.5);
}

#[test]
fn supports_unary_minus() {
    assert_close(evaluate("-5 + 2").unwrap(), -3.0);
}

#[test]
fn power_is_right_associative() {
    assert_close(evaluate("2^3^2").unwrap(), 512.0);
}

#[test]
fn unary_minus_binds_weaker_than_power() {
    assert_close(evaluate("-2^2").unwrap(), -4.0);
}

#[test]
fn supports_modulo() {
    assert_close(evaluate("7 % 4").unwrap(), 3.0);
}

#[test]
fn supports_sqrt() {
    assert_close(evaluate("sqrt(9)").unwrap(), 3.0);
}

#[test]
fn supports_abs() {
    assert_close(evaluate("abs(-3.5)").unwrap(), 3.5);
}

#[test]
fn supports_rounding_functions() {
    assert_close(evaluate("round(2.6)").unwrap(), 3.0);
    assert_close(evaluate("round(-2.5)").unwrap(), -3.0);
    assert_close(evaluate("floor(2.9)").unwrap(), 2.0);
    assert_close(evaluate("ceil(2.1)").unwrap(), 3.0);
}

#[test]
fn supports_nested_functions() {
    assert_close(evaluate("sqrt(abs(-16))").unwrap(), 4.0);
}

#[test]
fn ignores_whitespace_and_parses_decimals() {
    assert_close(evaluate("  1.5 * 2  ").unwrap(), 3.0);
}

#[test]
fn rejects_division_by_zero() {
    assert_eq!(evaluate("1 / 0").unwrap_err(), CalcError::DivisionByZero);
}

#[test]
fn rejects_modulo_by_zero() {
    assert_eq!(evaluate("1 % 0").unwrap_err(), CalcError::ModuloByZero);
}

#[test]
fn rejects_negative_square_root() {
    assert_eq!(
        evaluate("sqrt(-1)").unwrap_err(),
        CalcError::NegativeSquareRoot
    );
}

#[test]
fn rejects_unknown_character() {
    assert_eq!(
        evaluate("2 & 3").unwrap_err(),
        CalcError::UnexpectedToken('&')
    );
}

#[test]
fn rejects_unclosed_parenthesis() {
    assert_eq!(
        evaluate("(1 + 2").unwrap_err(),
        CalcError::UnclosedParenthesis
    );
}

#[test]
fn rejects_empty_expression() {
    assert_eq!(evaluate("   ").unwrap_err(), CalcError::EmptyExpression);
}

#[test]
fn rejects_unknown_function() {
    assert_eq!(
        evaluate("foo(1)").unwrap_err(),
        CalcError::UnknownFunction("foo".to_string())
    );
}

#[test]
fn requires_parenthesis_after_function_name() {
    assert_eq!(
        evaluate("sqrt 9").unwrap_err(),
        CalcError::ExpectedParenthesis("sqrt".to_string())
    );
}

#[test]
fn rejects_incomplete_expression() {
    assert_eq!(evaluate("1 +").unwrap_err(), CalcError::UnexpectedEnd);
}

#[test]
fn formats_integers_without_decimal_part() {
    assert_eq!(format_value(3.0), "3");
    assert_eq!(format_value(-0.0), "0");
    assert_eq!(format_value(2.5), "2.5");
}
