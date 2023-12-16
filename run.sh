run::test() {
  RUSTFLAGS="${RUSTFLAGS} -A dead_code" \
    cargo test
}

run::cli() {
  RUSTFLAGS="${RUSTFLAGS} -A dead_code" \
    cargo run "$@"
}

run::build() {
  cargo build
}

run::sample() {
  ./sample/generate.sh
  run::cli sample/snap1 sample/snap2
}
