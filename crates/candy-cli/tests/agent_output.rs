use std::path::PathBuf;
use std::process::Command;

fn candy_exe() -> PathBuf {
    // Preferred: Cargo-provided runtime var
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_candy") {
        return PathBuf::from(p);
    }
    // Fallback: workspace target path (crates/candy-cli -> ../../target/debug/candy)
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.join("../../target/debug/candy")
}

#[test]
fn agent_mode_outputs_json_only() {
    // Trigger a predictable typecheck error (missing main)
    let tmp = std::env::temp_dir().join(format!(
        "candy_agent_output_test_{}_{}.candy",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::write(&tmp, "fn not_main() -> Unit { return; }").unwrap();

    let exe = candy_exe();
    let out = Command::new(&exe)
        .args(["check", "--agent", tmp.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(!out.status.success());

    let stdout = String::from_utf8(out.stdout).unwrap();
    let _v: serde_json::Value = serde_json::from_str(&stdout).expect("stdout must be valid JSON");

    // Ensure no accidental noise on stdout (JSON-only contract)
    assert!(stdout.trim_start().starts_with('{'));
}
