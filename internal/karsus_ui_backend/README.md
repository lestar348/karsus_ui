# karsus_ui_backend

Rust workspace с интеграцией [`lcdhat`](https://github.com/lestar348/lcdhat) через FFI для управления Waveshare 1.44" LCD HAT.

## Структура

- `crates/lcdhat-sys`: unsafe FFI-слой + сборка `liblcdhat.so` из submodule через CMake.
- `crates/lcdhat`: безопасная Rust-обертка (`LcdHat`, `Config`, `Key`, `Error`, API рисования/кнопок/подсветки).
- `vendor/lcdhat`: git submodule с C-библиотекой.
- `examples/lcdhat_basic.rs`: smoke-пример `init -> draw -> present -> poll_key`.

## Подготовка репозитория

```bash
git submodule update --init --recursive
```

## Настройки backend

`lcdhat-sys` читает переменную окружения `LCDHAT_BACKEND`.

Допустимые значения:

- `DEV` (по умолчанию)
- `WIRINGPI`
- `BCM2835`

Пример:

```bash
export LCDHAT_BACKEND=DEV
```

## Сборка

```bash
cargo check --workspace
```

Во время сборки `crates/lcdhat-sys/build.rs`:

1. Конфигурирует CMake для `vendor/lcdhat`.
2. Собирает и устанавливает `liblcdhat.so` в build-артефакты Cargo.
3. Линкует `lcdhat` и выставляет `rpath` на директорию с библиотекой.

## Запуск примера

```bash
cargo run --example lcdhat_basic
```

Если в вашей системе не подхватывается `rpath`, запустите с `LD_LIBRARY_PATH`:

```bash
LD_LIBRARY_PATH=<path-to-liblcdhat-dir> cargo run --example lcdhat_basic
```

## Использование в коде

```rust
use lcdhat::{color, Config, Font, LcdHat};

fn main() -> Result<(), lcdhat::Error> {
    let mut lcd = LcdHat::new(Config::default())?;
    lcd.clear(color::WHITE)?;
    lcd.draw_text(8, 8, "hello", Font::Font12, color::BLACK, color::WHITE)?;
    lcd.present()?;
    Ok(())
}
```

## Ограничения API (важно)

- Разрешен только один активный `LcdHat` одновременно.
- `Config` валидируется:
  - `width=128`, `height=128`
  - `rotation` только `0/90/180/270`
  - `backlight` только `0..=1024`
- `present_region(x0, y0, x1, y1)` использует **exclusive** границы для `x1/y1`.

## Запуск на Raspberry Pi

Для `DEV` backend обычно требуется `lgpio` и доступ к GPIO/SPI.

Если аппаратная инициализация не проходит, API может вернуть `LCDHAT_ERR_HW_INIT`.

## Полезные команды

```bash
cargo test -p lcdhat
cargo check --workspace
cargo run --example lcdhat_basic
```
