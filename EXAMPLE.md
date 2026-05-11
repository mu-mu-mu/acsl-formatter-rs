# Library Usage Example

Add dependency in your external project's `Cargo.toml`:

```toml
[dependencies]
acsl-formatter = { path = "../acsl-formatter-rs" }
```

Use it from Rust:

```rust
use acsl_formatter::{
    format_acsl_annotation, format_acsl_comment, format_acsl_file, format_expression,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c_file = r#"
/*@
  ensures (a) && (b || c);
*/
int f(int a, int b, int c) { return a; }
"#;
    let formatted_file = format_acsl_file(c_file);
    println!("formatted file:\n{formatted_file}");

    let ann = format_acsl_annotation("ensures (a) && (b || c);");
    println!("formatted annotation: {ann}");

    let comment = format_acsl_comment("/*@ ensures (a) && (b || c); */");
    println!("formatted comment:\n{comment}");

    let expr = format_expression("!(a == b)")?;
    println!("formatted expression: {expr}"); // a != b

    Ok(())
}
```
