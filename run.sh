set -e

echo "=========== START A NEW BUILD ======"
cargo build
#RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel resource/example.icml

RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel.exe --include include/core.icml -i nosuchfile.icml \
--run example/max.icml