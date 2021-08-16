export ANGO_PATH=$PWD/.ango
cargo run -- add .gitignore -a gitignore
cargo run -- add src/angofile.rs -a gitignore
cargo run -- add .gitignore
cargo run -- add Cargo.lock
cargo run -- add Cargo.toml
cargo run -- add src