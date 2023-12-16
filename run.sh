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

run::demo() {
  local which="$1"
  assert-demo "${which}"
  run::cli "demo/${which}1" "demo/${which}2"
}

run::demo-generate() {
  local which="$1"
  assert-demo "${which}"
  "./demo/${which}.sh"
}

assert-demo() {
  local which="$1"
  [[ -f "./demo/${which}.sh" ]] && return
  echo 'No such demo' 1>&2
  exit 1
}
