use acsl_formatter::format_expression;

#[test]
fn simplifies_negated_equality() {
    let out = format_expression("!(a == b)").expect("format");
    assert_eq!(out, "a != b");
}

#[test]
fn simplifies_negated_relational() {
    let out = format_expression("!(a < b)").expect("format");
    assert_eq!(out, "a >= b");
}

#[test]
fn simplifies_double_negation() {
    let out = format_expression("!!flag").expect("format");
    assert_eq!(out, "flag");
}

#[test]
fn keeps_non_comparison_negation() {
    let out = format_expression("!(a && b)").expect("format");
    assert_eq!(out, "!(a && b)");
}
