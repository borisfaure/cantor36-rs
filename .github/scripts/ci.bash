#!/usr/bin/env bash
# Script for running check on your rust projects.
set -e
set -x
set -u

declare -A KEYMAPS
KEYMAPS=(
    [0]="keymap_borisfaure"
)
declare -A EXAMPLES
EXAMPLES=(
    [0]="blinky_led"
)


run_doc() {
    rustup component add rust-docs
    cargo doc
}

run_fmt() {
    rustup component add rustfmt
    cargo fmt --check
}

run_clippy() {
    rustup component add clippy-preview
    for EXAMPLE in "${EXAMPLES[@]}"
    do
        cargo clippy --example "$EXAMPLE" -- -D warnings
    done
    cargo clippy -- -D warnings
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo clippy --no-default-features --features "$KEYMAP" -- -D warnings
    done
}

run_check() {
    for EXAMPLE in "${EXAMPLES[@]}"
    do
        cargo check --example "$EXAMPLE"
    done
    cargo check
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo check --no-default-features --features "$KEYMAP"
    done
}

run_test() {
    cargo test
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo test --no-default-features --features "$KEYMAP"
    done
}

run_build() {
    for EXAMPLE in "${EXAMPLES[@]}"
    do
        cargo build --example "$EXAMPLE"
    done
    cargo build
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo build --no-default-features --features "$KEYMAP"
    done
}

run_build_release() {
    for EXAMPLE in "${EXAMPLES[@]}"
    do
        cargo build --release --example "$EXAMPLE"
    done
    cargo build --release
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo build --release --no-default-features --features "$KEYMAP"
    done
}

case $1 in
    doc)
        run_doc
        ;;
    fmt)
        run_fmt
        ;;
    check)
        run_check
        ;;
    clippy)
        run_clippy
        ;;
    test)
        run_test
        ;;
    build)
        run_build
        ;;
    build-release)
        run_build_release
        ;;
esac
