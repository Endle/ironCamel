set -e

echo "=========== START A NEW BUILD ======"
cargo build
#RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel resource/example.icml

RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel.exe resource/example.icml