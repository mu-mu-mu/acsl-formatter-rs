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
    assert_eq!(output, expected, "fixture failed: {name}");

    let _ = fs::remove_file(&tmp);
}

fn run_diff_fixture(name: &str) {
    let base = Path::new("tests/fixtures");
    let input_path = base.join(format!("{name}.c"));
    let input = fs::read_to_string(&input_path).expect("read input fixture");

    let tmp = temp_path(&format!("{name}_diff"));
    fs::write(&tmp, &input).expect("write temp input");

    let output = Command::new(env!("CARGO_BIN_EXE_acslfmt"))
        .arg("--diff")
        .arg(&tmp)
        .output()
        .expect("run acslfmt --diff");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("diff utf8");
    assert!(stdout.contains("--- "));
    assert!(stdout.contains("+++ "));
    assert!(stdout.contains("-  ensures (a) && (b || c);"));
    assert!(stdout.contains("+  ensures a && (b || c);"));

    let after = fs::read_to_string(&tmp).expect("read file after diff run");
    assert_eq!(after, input, "--diff must not rewrite file");

    let _ = fs::remove_file(&tmp);
}

fn run_diff_nochange_fixture(name: &str) {
    let base = Path::new("tests/fixtures");
    let expected_path = base.join(format!("{name}.expected.c"));
    let expected = fs::read_to_string(&expected_path).expect("read expected fixture");

    let tmp = temp_path(&format!("{name}_diff_nochange"));
    fs::write(&tmp, &expected).expect("write temp input");

    let output = Command::new(env!("CARGO_BIN_EXE_acslfmt"))
        .arg("--diff")
        .arg(&tmp)
        .output()
        .expect("run acslfmt --diff");
    assert!(output.status.success());
    assert!(
        output.stdout.is_empty(),
        "no diff expected for already formatted file"
    );

    let after = fs::read_to_string(&tmp).expect("read file after diff run");
    assert_eq!(after, expected, "--diff must not rewrite file");

    let _ = fs::remove_file(&tmp);
}

#[test]
fn formats_all_fixtures() {
    let base = Path::new("tests/fixtures");
    let mut names = Vec::new();
    for entry in fs::read_dir(base).expect("read fixtures dir") {
        let entry = entry.expect("read dir entry");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("c") {
            continue;
        }
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };
        if file_name.ends_with(".expected.c") {
            continue;
        }
        let stem = file_name.strip_suffix(".c").expect("stem");
        let expected = base.join(format!("{stem}.expected.c"));
        if expected.exists() {
            names.push(stem.to_string());
        }
    }
    names.sort();
    assert!(!names.is_empty(), "no fixtures found");
    for name in names {
        run_fixture(&name);
        println!("fixture ok: {name}");
    }
}

#[test]
fn diff_mode_prints_changes_without_rewrite() {
    run_diff_fixture("basic");
}

#[test]
fn diff_mode_prints_nothing_when_no_changes() {
    run_diff_nochange_fixture("basic");
}
