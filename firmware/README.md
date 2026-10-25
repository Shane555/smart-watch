# Firmware

Rust firmware for the ESP32-C3, using the ESP-IDF `std` route (`esp-idf-svc` / `esp-idf-hal`).
Target: `riscv32imc-esp-espidf`, ESP-IDF v5.3.2 (set in [`.cargo/config.toml`](.cargo/config.toml)).

> Run every `cargo` command from **inside this folder**. Cargo picks up `.cargo/config.toml`
> (target, linker, runner, `build-std`) and rustup picks up `rust-toolchain.toml` (nightly + `rust-src`)
> from the current directory, so running from the repo root won't find them.

## Prerequisites

```sh
rustup toolchain install nightly --component rust-src   # rust-toolchain.toml selects it
cargo install ldproxy espflash
```

ESP-IDF itself is downloaded automatically by `embuild` on the first build, into `firmware/.embuild/`
(ignored by Git). It needs `git`, `python3` and `python3-venv` on your PATH. The first build takes a while.

## Build and flash

```sh
cd firmware
cargo build            # debug
cargo build --release
cargo run              # build, flash and open the serial monitor (espflash)
```

## Checks (same as CI)

```sh
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --workspace -- -D warnings
```
