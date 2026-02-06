use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn candy_exe() -> PathBuf {
    if let Ok(p) = std::env::var("CARGO_BIN_EXE_candy") {
        return PathBuf::from(p);
    }
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest.join("../../target/debug/candy")
}

fn unique_tmp_path(prefix: &str) -> PathBuf {
    let n = COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    std::env::temp_dir().join(format!("{prefix}_{pid}_{n}.candy"))
}

fn run_agent(src: &str, tag: &str) -> serde_json::Value {
    let path = unique_tmp_path(&format!("candy_agent_{tag}"));
    fs::write(&path, src).unwrap();

    let exe = candy_exe();
    let out = Command::new(&exe)
        .args(["check", "--agent", path.to_str().unwrap()])
        .output()
        .unwrap();

    assert!(
        !out.status.success(),
        "expected failure exit code, got success. stdout={:?} stderr={:?}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr),
    );

    let stdout = String::from_utf8(out.stdout).unwrap();
    serde_json::from_str(&stdout).expect("stdout must be valid JSON")
}

fn diag_codes(v: &serde_json::Value) -> Vec<String> {
    v.get("diagnostics")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| {
                    let s = x
                        .get("code")
                        .and_then(|c| c.as_str())
                        .or_else(|| x.get("error").and_then(|c| c.as_str()));
                    s.map(|s| s.trim().to_string())
                })
                .collect()
        })
        .unwrap_or_default()
}

fn has_code(v: &serde_json::Value, want: &str) -> bool {
    let want = want.trim();
    diag_codes(v).iter().any(|c| c == want)
}

#[test]
fn agent_reports_secret_copy() {
    let v = run_agent(
        r#"
fn main() -> Unit {
  let a: secret Int = 1;
  let b: secret Int = a;
  return;
}
"#,
        "secret_copy",
    );

    assert!(
        has_code(&v, "secret-copy"),
        "expected secret-copy. got codes={:?} full_json={}",
        diag_codes(&v),
        v
    );
}

#[test]
fn agent_reports_use_after_move() {
    let v = run_agent(
        r#"
fn main() -> Unit {
  let a: secret Int = 1;
  let b: secret Int = move(a);
  a;
  return;
}
"#,
        "use_after_move",
    );

    assert!(
        has_code(&v, "use-after-move"),
        "expected use-after-move. got codes={:?} full_json={}",
        diag_codes(&v),
        v
    );
}

#[test]
fn agent_reports_secret_branch() {
    let v = run_agent(
        r#"
fn main() -> Unit {
  let s: secret Bool = true;
  if (s) { return; } else { return; }
}
"#,
        "secret_branch",
    );

    assert!(
        has_code(&v, "secret-branch"),
        "expected secret-branch. got codes={:?} full_json={}",
        diag_codes(&v),
        v
    );
}
