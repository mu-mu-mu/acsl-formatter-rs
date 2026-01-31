use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn temp_path(name: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time")
        .as_nanos();
    path.push(format!("acslfmt_{name}_{nonce}.c"));
    path
}

#[test]
fn formats_acsl_annotations_in_place() {
    let path = temp_path("basic");
    let input = "int foo(int a){\n/*@ (a) && (b || c) ; */\nreturn a;\n}\n";
    fs::write(&path, input).expect("write");

    let status = Command::new(env!("CARGO_BIN_EXE_acslfmt"))
        .arg(&path)
        .status()
        .expect("run acslfmt");
    assert!(status.success());

    let output = fs::read_to_string(&path).expect("read");
    assert_eq!(output, "int foo(int a){\n/*@ a && (b || c); */\nreturn a;\n}\n");

    let _ = fs::remove_file(&path);
}

#[test]
fn preserves_non_acsl_content() {
    let path = temp_path("plain");
    let input = "int main(){\n// regular comment\nreturn 0;\n}\n";
    fs::write(&path, input).expect("write");

    let status = Command::new(env!("CARGO_BIN_EXE_acslfmt"))
        .arg(&path)
        .status()
        .expect("run acslfmt");
    assert!(status.success());

    let output = fs::read_to_string(&path).expect("read");
    assert_eq!(output, input);

    let _ = fs::remove_file(&path);
}
