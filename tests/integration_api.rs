use acsl_formatter::format_acsl_file;

#[test]
fn formats_block_annotation_end_to_end() {
    let input = r#"/*@
loop invariant (i) < (cpu + 1) ==> (\forall integer __i; 0 <= __i < (i) ==> (
(sched_domain_span ( sd ))->bits[__i] ? !(__i == cpu) ==> !__need_resched :  1));
*/"#;

    let out = format_acsl_file(input);

    assert!(out.contains("loop invariant i < cpu + 1 ==>"));
    assert!(out.contains("sched_domain_span(sd)->bits[__i]"));
    assert!(out.contains("__i != cpu"));
    assert!(out.contains("==>"));
    assert!(out.contains("!__need_resched"));
}

#[test]
fn leaves_non_acsl_comments_unchanged() {
    let input = r#"int main() {
// regular comment
return 0;
}"#;
    let out = format_acsl_file(input);
    assert_eq!(out, input);
}
