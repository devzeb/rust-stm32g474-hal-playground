This repository is my playground for learning embedded rust.
It implements a HAL for the stm32g474 chip.

# Prerequisites
- `rustup target add thumbv7em-none-eabihf`

# How to build
- `cargo build`
The elf file is then located in `target/thumbv7em-none-eabihf/debug/app`

# How to debug
## Vscode on Ubuntu
- Install the cortex-debug extension
- Install `sudo apt install gdb-multiarch`
- Open vscode user settings and add the following:
```json
"cortex-debug.gdbPath": "/usr/bin/gdb-multiarch",
```