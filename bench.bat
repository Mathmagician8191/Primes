cargo clean
cargo run --release -p bench
ls -l target/release
sha256sum target/release/bench