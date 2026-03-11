use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (diff_only, path) = match args.as_slice() {
        [_, path] => (false, path.as_str()),
        [_, flag, path] if flag == "--diff" || flag == "-d" => (true, path.as_str()),
        _ => {
            eprintln!("usage: acslfmt [--diff|-d] <path>");
            process::exit(2);
        }
    };

    let input = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            process::exit(1);
        }
    };

    let formatted = acsl_formatter::format_acsl_file(&input);
    if formatted == input {
        return;
    }

    if diff_only {
        let diff = unified_diff(path, &input, &formatted);
        print!("{diff}");
        return;
    }

    if let Err(err) = fs::write(path, formatted) {
        eprintln!("failed to write {path}: {err}");
        process::exit(1);
    }
}

fn unified_diff(path: &str, old: &str, new: &str) -> String {
    let old_lines: Vec<&str> = old.split('\n').collect();
    let new_lines: Vec<&str> = new.split('\n').collect();
    let max = old_lines.len().max(new_lines.len());
    let mut out = String::new();
    out.push_str(&format!("--- {path}\n"));
    out.push_str(&format!("+++ {path}\n"));
    out.push_str(&format!("@@ -1,{} +1,{} @@\n", old_lines.len(), new_lines.len()));

    for i in 0..max {
        match (old_lines.get(i), new_lines.get(i)) {
            (Some(a), Some(b)) if a == b => {
                out.push(' ');
                out.push_str(a);
                out.push('\n');
            }
            (Some(a), Some(b)) => {
                out.push('-');
                out.push_str(a);
                out.push('\n');
                out.push('+');
                out.push_str(b);
                out.push('\n');
            }
            (Some(a), None) => {
                out.push('-');
                out.push_str(a);
                out.push('\n');
            }
            (None, Some(b)) => {
                out.push('+');
                out.push_str(b);
                out.push('\n');
            }
            (None, None) => {}
        }
    }

    out
}
