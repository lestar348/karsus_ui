---
name: karsus-ui-backend-lcdhat
description: Используй этот skill при работе с Rust backend проекта karsus_ui для интеграции и использования lcdhat (Waveshare 1.44 LCD HAT): обзор workspace, публичный API crates/lcdhat, ограничения платформы/конфигурации, и безопасные правила изменений.
---

# karsus_ui backend + lcdhat

## Когда использовать

Используй этот skill, когда задача относится к репозиторию `internal/karsus_ui_backend` или к API crate `lcdhat`:
- добавить/исправить использование дисплея и кнопок;
- обновить пример, draw/present цикл, обработку клавиш;
- изменить `Config`/валидацию/ошибки;
- править FFI build (`lcdhat-sys/build.rs`) или backend (`LCDHAT_BACKEND`).

## Карта репозитория

- `internal/karsus_ui_backend/Cargo.toml`: workspace root.
- `internal/karsus_ui_backend/src/lib.rs`: `pub use lcdhat::*;` (реэкспорт API).
- `internal/karsus_ui_backend/crates/lcdhat`: безопасная Rust-обертка.
- `internal/karsus_ui_backend/crates/lcdhat-sys`: FFI и сборка C-библиотеки через CMake.
- `internal/karsus_ui_backend/vendor/lcdhat`: C submodule `lcdhat`.
- `internal/karsus_ui_backend/examples/lcdhat_basic.rs`: канонический smoke-flow.

## Публичные сущности (`crates/lcdhat`)

### Структуры и enum

- `pub struct LcdHat`
- `pub struct Config`
  - поля: `width`, `height`, `rotation`, `background`, `foreground`, `backlight`, `poll_debounce_ms`
- `pub enum Font`: `Font8`, `Font12`, `Font16`, `Font20`, `Font24`
- `pub enum Key`: `Up`, `Down`, `Left`, `Right`, `Press`, `K1`, `K2`, `K3`
- `pub struct KeyEvent { key: Key, pressed: bool }`
- `pub enum Error`
- `pub type Result<T> = core::result::Result<T, Error>`
- Константы: `LCD_WIDTH=128`, `LCD_HEIGHT=128`, `MAX_BACKLIGHT=1024`
- `pub mod color`: `WHITE`, `BLACK`, `BLUE`, `RED`, `MAGENTA`, `GREEN`, `CYAN`, `YELLOW`, `GRAY`

### Публичные методы

- `Config::validate(self) -> Result<Self>`
- `LcdHat::new(config: Config) -> Result<Self>`
- `LcdHat::clear(&mut self, color: u16) -> Result<()>`
- `LcdHat::present(&mut self) -> Result<()>`
- `LcdHat::present_region(&mut self, x0, y0, x1, y1) -> Result<()>`
- `LcdHat::draw_pixel(&mut self, x, y, color) -> Result<()>`
- `LcdHat::draw_line(&mut self, x0, y0, x1, y1, color, width) -> Result<()>`
- `LcdHat::draw_rect(&mut self, x0, y0, x1, y1, color, line_width, filled) -> Result<()>`
- `LcdHat::draw_text(&mut self, x, y, text, font, fg, bg) -> Result<()>`
- `LcdHat::set_backlight(&mut self, value) -> Result<()>`
- `LcdHat::poll_key(&mut self) -> Result<Option<KeyEvent>>`
- `LcdHat::sleep_ms(ms: u32)`

## Инварианты и ограничения

- Поддерживается только Linux для реальной инициализации `LcdHat::new`; иначе `Error::UnsupportedPlatform`.
- Разрешен только один активный экземпляр `LcdHat` одновременно (`Error::AlreadyInitialized`).
- Валидная конфигурация только:
  - `width == 128`
  - `height == 128`
  - `rotation` в `{0, 90, 180, 270}`
  - `backlight` в `0..=1024`
- `present_region` требует строгие границы: `x0 < x1`, `y0 < y1`, и `x1/y1` не выходят за размеры дисплея.
- `draw_text` не принимает строки с `\0` (получите `Error::InvalidString`).
- `LCDHAT_BACKEND` допускает только `DEV`, `WIRINGPI`, `BCM2835`.

## Что нужно делать

- Начинать сценарий с `LcdHat::new(Config::default())` или валидированного `Config`.
- После операций рисования вызывать `present()` или `present_region()`.
- В loop опроса кнопок обрабатывать `None` через короткий sleep (`LcdHat::sleep_ms(10)`), чтобы не делать busy-spin.
- Перед изменениями FFI слоя проверять совместимость `bindings_pregenerated.rs` с заголовком `vendor/lcdhat/include/lcdhat/lcdhat.h`.
- Для проверки использовать:
  - `cargo check --workspace`
  - `cargo test -p lcdhat`
  - `cargo run --example lcdhat_basic`

## Что нельзя делать

- Нельзя создавать несколько одновременно живых `LcdHat`.
- Нельзя ослаблять проверку `Config::validate` без явной причины и обновления тестов.
- Нельзя менять сигнатуры публичных методов `lcdhat` без синхронного обновления:
  - `README.md`
  - `examples/lcdhat_basic.rs`
  - (при необходимости) внешних пользователей через root re-export (`src/lib.rs`).
- Нельзя добавлять платформенно-зависимый код в safe API без явной ветки `cfg(target_os = "linux")` и fallback/ошибки.
- Нельзя вводить `unsafe` в `crates/lcdhat` без комментария про безопасность (почему это sound).

## Рабочий порядок для агента

1. Проверить область изменения: `crates/lcdhat` (safe API) vs `crates/lcdhat-sys` (FFI/build).
2. Если меняется поведение API, сначала зафиксировать инварианты (один инстанс, bounds, platform).
3. Сделать минимальные изменения и прогнать релевантные команды проверки.
4. В отчете указывать затронутые файлы и поведенческие изменения.

## Быстрый шаблон использования

```rust
use lcdhat::{color, Config, Font, LcdHat, Result};

fn demo() -> Result<()> {
    let mut lcd = LcdHat::new(Config::default())?;
    lcd.clear(color::WHITE)?;
    lcd.draw_text(8, 8, "hello", Font::Font12, color::BLACK, color::WHITE)?;
    lcd.present()?;
    Ok(())
}
```
