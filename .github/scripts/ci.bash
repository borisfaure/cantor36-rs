#!/usr/bin/env bash
# Script for running check on your rust projects.
set -e
set -x
set -u

declare -A KEYMAPS
KEYMAPS=(
    "keymap_borisfaure"
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
    cargo clippy -- -D warnings
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo clippy --no-default-features --features "$FEAT,$KEYMAP" -- -D warnings
    done
}

run_check() {
    cargo check
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo check --no-default-features --features "$FEAT,$KEYMAP"
    done
}

run_test() {
    cargo test
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo test --no-default-features --features "$FEAT,$KEYMAP"
    done
}

run_build() {
    cargo build
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo build --no-default-features --features "$FEAT,$KEYMAP"
    done
}

run_build_release() {
    cargo build --release
    for KEYMAP in "${KEYMAPS[@]}"
    do
        cargo build --release --no-default-features --features "$FEAT,$KEYMAP"
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
