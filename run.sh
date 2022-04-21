set -e

echo "=========== START A NEW BUILD ======"
cargo build
#RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel resource/str_basic.icml

RUST_BACKTRACE=1 RUST_LOG=info  target/debug/ironcamel.exe \
--run example/io_read_then_sort.icml \
--include include/core.icml -i include/stdlib.icml \
