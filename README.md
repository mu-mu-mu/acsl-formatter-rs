# acsl-formatter-rs

Very small formatter for ACSL annotations in C files.

## What it changes

`acslfmt` rewrites only ACSL comments (`/*@ ... */` and `//@ ...`).
Normal C code and normal comments are kept as-is.

Supported modifications:

- Normalize expression spacing (`a+b` -> `a + b`, `f(x,y)` -> `f(x, y)`)
- Normalize member/index spacing (`a -> s [ 5 ]` -> `a->s[5]`)
- Remove redundant parentheses (`(a) && (b || c)` -> `a && (b || c)`)
- Keep required parentheses for precedence/associativity
- Format known ACSL clauses:
  `requires`, `ensures`, `assumes`, `assigns`, `assert`, `loop invariant`, `behavior <name>:`
- Indent `behavior` sub-clauses
- Wrap long clauses to about 80 columns with hanging indentation
- Add/normalize semicolon layout in block annotations
- Keep annotations unchanged if parsing fails (best effort / safe fallback)

Supported expression forms inside those clauses:

- Unary/binary operators (`!`, `+`, `-`, `*`, `/`, `%`, comparisons, `&&`, `||`, `==>`, `<==>`)
- Ternary (`cond ? a : b`)
- Calls, indexing, member access (`f(x)`, `a[i]`, `p->x`, `s.x`)
- Quantifiers (`\forall`, `\exists`)

## Usage

```bash
cargo run --bin acslfmt -- path/to/file.c
```

The file is updated in place.
