use std::fs;
use std::path::{Path, PathBuf};
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

fn run_fixture(name: &str) {
    let base = Path::new("tests/fixtures");
    let input_path = base.join(format!("{name}.c"));
    let expected_path = base.join(format!("{name}.expected.c"));

    let input = fs::read_to_string(&input_path).expect("read input fixture");
    let expected = fs::read_to_string(&expected_path).expect("read expected fixture");

    let tmp = temp_path(name);
    fs::write(&tmp, input).expect("write temp input");

    let status = Command::new(env!("CARGO_BIN_EXE_acslfmt"))
        .arg(&tmp)
        .status()
        .expect("run acslfmt");
    assert!(status.success());

    let output = fs::read_to_string(&tmp).expect("read output");
    assert_eq!(output, expected);

    let _ = fs::remove_file(&tmp);
}

#[test]
fn formats_acsl_annotations_in_place() {
    run_fixture("basic");
}

#[test]
fn preserves_non_acsl_content() {
    run_fixture("preserve");
}
