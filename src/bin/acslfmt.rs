use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: acslfmt <path>");
        process::exit(2);
    }

    let path = &args[1];
    let input = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            process::exit(1);
        }
    };

    let formatted = acsl_formatter::format_acsl_file(&input);
    if formatted != input {
        if let Err(err) = fs::write(path, formatted) {
            eprintln!("failed to write {path}: {err}");
            process::exit(1);
        }
    }
}
