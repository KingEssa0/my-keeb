# Keeb65 firmware

The firmware targets the original Raspberry Pi Pico (RP2040). It uses RMK 0.8.2 and the wiring in [matrix.csv](../matrix.csv).

## Flash

1. Unplug the Pico. Hold **BOOTSEL** while plugging it back in with a USB data cable.
2. When **RPI-RP2** appears in Finder, copy [Keeb65.uf2](Keeb65.uf2) to it. The drive ejects and the Pico restarts automatically.
3. Test every key once the board is assembled. Hold **Fn** for F1–F12 across the number row, backtick on Esc, Home on Page Up, and Insert on Backspace.

## Rebuild

With Rust installed, run these commands from this folder.

```sh
rustup target add thumbv6m-none-eabi
cargo install elf2uf2-rs --version 2.2.0 --locked
cargo build --release --locked
elf2uf2-rs target/thumbv6m-none-eabi/release/keeb65 Keeb65.uf2
```

[Raspberry Pi BOOTSEL instructions](https://www.raspberrypi.com/documentation/microcontrollers/pico-series.html#bootsel-mode) · [RMK RP2040 example](https://github.com/HaoboGu/rmk/tree/rmk-v0.8.2/examples/use_config/rp2040)
