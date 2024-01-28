#!/bin/bash

# Run tests
run::test() {
  RUSTFLAGS="${RUSTFLAGS} -A dead_code" \
    cargo test
}

# Run the CLI on the fly
run::cli() {
  RUSTFLAGS="${RUSTFLAGS} -A dead_code" \
    cargo run -- "$@"
}

# Compile project
run::build() {
  cargo build --release
}

# Run one of the demo folders
run::demo() {
  local which="$1"
  shift 1
  assert-demo "${which}"
  run::cli "$@" "demo/${which}1" "demo/${which}2"
}

# Format all code
run::format() {
  rustfmt src/**
}

# Generate demo folders
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
