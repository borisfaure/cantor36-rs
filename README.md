![CI](https://github.com/borisfaure/cantor36-rs/actions/workflows/ci.yml/badge.svg)

# Rust Firmware for the Cantor36 keyboard

This firmware written in Rust is targetted for the
[Cantor36 keyboard](https://github.com/borisfaure/cantor36) built with a
STM32F401CDUx/STM32F411CDUx MCU on a [WeAct Black Pill](https://stm32-base.org/boards/STM32F411CEU6-WeAct-Black-Pill-V2.0.html).

It is based on the [Keyberon library](https://github.com/TeXitoi/keyberon).

## Features

- Multi layers keymaps
- Multiple keymaps
- Hold Tap actions
- Sequences

## What's missing

- No support for controlling the mouse
- One Shot Actions
- ...

## Installing the needed tools

Considering one has rust installed by [rustup.rs](https://rustup.rs), then
one has to run the following commands:

```shell
cargo install cargo-binutils
rustup component add llvm-tools-preview
cargo install probe-rs --features cli
```

## Compile & Flashing

The possible keymaps are:

- `keymap_basic`
- `keymap_borisfaure`
- `keymap_test`


In order to generate and install the firmware for the keymap `keymap_basic`
for the `left` side using [probe-rs](https://probe.rs/):

```shell
cargo f --release --features="left,keymap_borisfaure"
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

