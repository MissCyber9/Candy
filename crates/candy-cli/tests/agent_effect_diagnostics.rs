use std::process::Command;

fn candy_bin() -> String {
    // Cargo sets this env var for integration tests.
    env!("CARGO_BIN_EXE_candy").to_string()
}

fn write_temp(src: &str) -> tempfile::NamedTempFile {
    let mut f = tempfile::NamedTempFile::new().expect("tempfile");
    std::io::Write::write_all(&mut f, src.as_bytes()).expect("write");
    f
}

#[test]
fn agent_reports_undeclared_effect_for_log() {
    let src = r#"
fn main() -> Unit {
  log("x");
  return;
}
"#;
    let f = write_temp(src);

    let out = Command::new(candy_bin())
        .args(["check", "--agent", f.path().to_str().unwrap()])
        .output()
        .expect("run candy");

    assert!(out.status.code().unwrap_or(1) != 0, "should fail");
    let s = String::from_utf8(out.stdout).expect("utf8 stdout");

    let v: serde_json::Value = serde_json::from_str(&s).expect("stdout is json");
    let diags = v.get("diagnostics").and_then(|d| d.as_array()).unwrap();

    assert!(
        diags
            .iter()
            .any(|d| d.get("code").and_then(|c| c.as_str()) == Some("undeclared-effect")),
        "expected undeclared-effect"
    );
}

#[test]
fn agent_reports_effect_leak_for_call() {
    let src = r#"
fn g() -> Unit effects(io) {
  log("x");
  return;
}

fn main() -> Unit {
  g();
  return;
}
"#;
    let f = write_temp(src);

    let out = Command::new(candy_bin())
        .args(["check", "--agent", f.path().to_str().unwrap()])
        .output()
        .expect("run candy");

    assert!(out.status.code().unwrap_or(1) != 0, "should fail");
    let s = String::from_utf8(out.stdout).expect("utf8 stdout");

    let v: serde_json::Value = serde_json::from_str(&s).expect("stdout is json");
    let diags = v.get("diagnostics").and_then(|d| d.as_array()).unwrap();

    assert!(
        diags
            .iter()
            .any(|d| d.get("code").and_then(|c| c.as_str()) == Some("effect-leak")),
        "expected effect-leak"
    );
}
