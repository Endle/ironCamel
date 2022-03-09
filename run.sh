set -e

cargo build
RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel resource/example.icml