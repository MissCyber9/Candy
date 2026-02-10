.PHONY: fmt clippy test audit

fmt:
cargo fmt --all

clippy:
cargo clippy --workspace --all-targets -- -D warnings

test:
cargo test --workspace

audit:
mkdir -p audit/latest
rustc -Vv > audit/latest/rustc.txt
cargo -V > audit/latest/cargo.txt
cargo metadata --format-version=1 > audit/latest/cargo-metadata.json
cargo tree --workspace > audit/latest/cargo-tree.txt
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings > audit/latest/clippy.txt
cargo test --workspace > audit/latest/tests.txt
git rev-parse HEAD > audit/latest/git-sha.txt
git status --porcelain > audit/latest/git-status.txt
