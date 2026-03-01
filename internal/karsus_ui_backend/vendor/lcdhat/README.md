# lcdhat

`lcdhat` is a C shared library for the Waveshare 1.44 inch LCD HAT on Raspberry Pi.
It provides a stable public API for initialization, drawing, framebuffer presentation,
GPIO key polling, and backlight control.

## Prerequisites

Install one backend library on Raspberry Pi:

- `DEV` backend: `lgpio`
- `WIRINGPI` backend: `wiringPi`
- `BCM2835` backend: `bcm2835`

## Build + run example (DEV backend / `-DLCDHAT_BACKEND=DEV`)

```sh
cmake -S . -B build-dev -DLCDHAT_BACKEND=DEV
cmake --build build-dev -j

# run example from build tree
LD_LIBRARY_PATH=build-dev ./build-dev/examples/basic_demo/lcdhat_basic_demo
```

## Build + run example (WIRINGPI backend / `-DLCDHAT_BACKEND=WIRINGPI`)

```sh
cmake -S . -B build-wiringpi -DLCDHAT_BACKEND=WIRINGPI
cmake --build build-wiringpi -j

# run example from build tree
LD_LIBRARY_PATH=build-wiringpi ./build-wiringpi/examples/basic_demo/lcdhat_basic_demo
```

## Build + run example (BCM2835 backend / `-DLCDHAT_BACKEND=BCM2835`)

```sh
cmake -S . -B build-bcm2835 -DLCDHAT_BACKEND=BCM2835
cmake --build build-bcm2835 -j

# run example from build tree
LD_LIBRARY_PATH=build-bcm2835 ./build-bcm2835/examples/basic_demo/lcdhat_basic_demo
```

## Install

```sh
cmake --install build-dev --prefix ./build-dev/install
```

This installs:
- library: `liblcdhat.so`
- public header: `include/lcdhat/lcdhat.h`

## Logging

Library has two log levels:
- `ERROR` - always enabled.
- `DEBUG` - enabled by environment variable.

Environment variables:
- `LCDHAT_LOG_DEBUG` - enables debug logs when set to `1`, `true`, `yes`, `on`, or `debug`.
- `LCDHAT_LOG_FILE` - path to log file. If empty or file cannot be opened, logs fall back to stdout.

Examples:

```sh
# errors only (default), stdout
LD_LIBRARY_PATH=build-dev ./build-dev/examples/basic_demo/lcdhat_basic_demo

# debug + errors to stdout
LCDHAT_LOG_DEBUG=1 LD_LIBRARY_PATH=build-dev ./build-dev/examples/basic_demo/lcdhat_basic_demo

# debug + errors to file
LCDHAT_LOG_DEBUG=1 LCDHAT_LOG_FILE=/tmp/lcdhat.log \
LD_LIBRARY_PATH=build-dev ./build-dev/examples/basic_demo/lcdhat_basic_demo
```

## v1 limitation

Due to global state in the underlying Waveshare backend, v1 supports only one active
`lcdhat_ctx_t` at a time.
