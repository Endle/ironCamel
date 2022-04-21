set -e

echo "=========== START A NEW BUILD ======"
cargo build
#RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel resource/str_basic.icml

RUST_BACKTRACE=1 RUST_LOG=warn  target/debug/ironcamel.exe \
--run example/file_io_write.icml \
--include include/core.icml -i include/stdlib.icml \
