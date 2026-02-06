use std::fs;
use std::process::Command;

#[test]
fn agent_mode_outputs_json_only() {
    // Create temp file in target dir
    let dir = std::env::temp_dir();
    let path = dir.join("candy_agent_test.candy");
    fs::write(&path, "main() -> Unit { return; }").unwrap(); // invalid (missing fn)

    let exe = env!("CARGO_BIN_EXE_candy-cli");
    let out = Command::new(exe)
        .args(["check", "--agent", path.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(!out.status.success());

    let stdout = String::from_utf8(out.stdout).unwrap();
    let stderr = String::from_utf8(out.stderr).unwrap();

    // stdout must be JSON
    let v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout must be valid JSON");

    assert!(v.get("diagnostics").is_some());

    // stderr should not contain JSON requirement, but can contain nothing or logs.
    // Most importantly: JSON should not be printed to stderr in agent mode.
    assert!(!stderr.contains("\"diagnostics\""));
}
