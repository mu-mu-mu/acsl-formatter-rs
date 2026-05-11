use acsl_formatter::{format_acsl_annotation, format_acsl_comment, format_expression};

#[test]
fn removes_redundant_parentheses() {
    let out = format_acsl_annotation("ensures (a) && (b || c);");
    assert_eq!(out, "ensures a && (b || c)");
}

#[test]
fn removes_redundant_parentheses_in_requires_clause() {
    let out = format_acsl_annotation("requires (a == 1);");
    assert_eq!(out, "requires a == 1");
}

#[test]
fn removes_redundant_parentheses_around_requires_condition() {
    let out = format_acsl_annotation("requires (a && b);");
    assert_eq!(out, "requires a && b");
}

#[test]
fn removes_redundant_parentheses_around_bitand_requires_condition() {
    let out = format_acsl_annotation("requires (a & b);");
    assert_eq!(out, "requires a & b");
}

#[test]
fn removes_redundant_parentheses_around_bitor_requires_condition() {
    let out = format_acsl_annotation("requires (a | b);");
    assert_eq!(out, "requires a | b");
}

#[test]
fn formats_block_comment_annotation() {
    let out = format_acsl_comment("/*@ requires (a + b) * (c) > 0; */");
    assert_eq!(out, "/*@\n  requires (a + b) * c > 0;\n*/");
}

#[test]
fn removes_redundant_parentheses_in_block_comment_requires_clause() {
    let out = format_acsl_comment("/*@ requires (a == 1); */");
    assert_eq!(out, "/*@\n  requires a == 1;\n*/");
}

#[test]
fn normalizes_call_index_member_spacing() {
    let out = format_expression("f(a -> s [ 5 ])").expect("format");
    assert_eq!(out, "f(a->s[5])");
}

#[test]
fn normalizes_assigns_list_spacing() {
    let out = format_acsl_annotation("assigns a[ i ], b  ;");
    assert_eq!(out, "assigns a[i], b");
}

#[test]
fn keeps_required_parentheses_for_precedence() {
    let out = format_expression("(a + b) * c").expect("format");
    assert_eq!(out, "(a + b) * c");
}

#[test]
fn keeps_relational_operand_parenthesized_inside_equality() {
    let out = format_expression("s == (10 > num)").expect("format");
    assert_eq!(out, "s == (10 > num)");
}

#[test]
fn keeps_bitand_operand_parenthesized_inside_equality() {
    let out = format_expression("a == (b & c)").expect("format");
    assert_eq!(out, "a == (b & c)");
}

#[test]
fn keeps_bitor_operand_parenthesized_inside_bitand() {
    let out = format_expression("a & (b | c)").expect("format");
    assert_eq!(out, "a & (b | c)");
}

#[test]
fn keeps_and_operands_parenthesized_inside_or_for_readability() {
    let out = format_expression("(a && b) || (a && c)").expect("format");
    assert_eq!(out, "(a && b) || (a && c)");
}

#[test]
fn formats_simple_quantifier() {
    let out = format_acsl_annotation(r"ensures \forall i; (0 <= i) && (i < n) ==> a[i] == b[i];");
    assert_eq!(out, r"ensures \forall i; 0 <= i && i < n ==> a[i] == b[i]");
}
