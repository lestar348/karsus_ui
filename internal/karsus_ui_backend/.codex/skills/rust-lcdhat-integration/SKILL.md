---
name: rust-lcdhat-integration
description: Интеграция C-библиотеки lcdhat (Waveshare 1.44 inch LCD HAT) в Rust-проекты на Raspberry Pi через FFI. Используй этот skill, когда нужно подключить lcdhat в Cargo-проект, настроить сборку C-кода (DEV/WIRINGPI/BCM2835 backend), сгенерировать bindings, сделать безопасную Rust-обертку и корректно использовать API отрисовки/кнопок/подсветки.
---

# Rust lcdhat integration

## Когда использовать

Используй этот skill, если пользователь просит:

- подключить эту библиотеку `c` к Rust проекту;
- настроить `build.rs` для сборки `liblcdhat.so`;
- сделать FFI bindings к `include/lcdhat/lcdhat.h`;
- написать безопасный Rust-слой поверх C API;
- разобраться с жизненным циклом контекста, отрисовкой и кнопками.

## Важные факты API (из кода библиотеки)

- Публичный API: `include/lcdhat/lcdhat.h`.
- Размер дисплея фиксирован на `128x128`; `lcdhat_init` вернет ошибку для других `width/height`.
- Разрешенные повороты: `0`, `90`, `180`, `270`.
- В v1 разрешен только один активный `lcdhat_ctx_t` одновременно (`LCDHAT_ERR_STATE` при втором `init`).
- `lcdhat_present_region(x0, y0, x1, y1)`: `x1/y1` являются эксклюзивной границей.
- `lcdhat_set_backlight`: допустим диапазон `0..=1024`.
- `lcdhat_poll_key` отдает событие только на фронт нажатия (debounce через `poll_debounce_ms`).
- Возможен stub-режим для backend `DEV`, если `lgpio` не найден (сборка проходит, железо не инициализируется полноценно).

## Справочник API (`lcdhat.h`)

- Полный reference вынесен в `references/lcdhat-api.md`.
- Загружай `references/lcdhat-api.md` когда нужно:
  - описать все публичные типы/enum/структуры/функции;
  - реализовать или ревьюить Rust FFI bindings;
  - проверить точные контракты аргументов и ограничения API.

## Базовый workflow интеграции в Rust

1. Собрать C-библиотеку через CMake из Rust (`cmake` crate) с выбранным backend.
2. Сгенерировать Rust bindings из `lcdhat.h` (`bindgen`) или использовать заранее подготовленные.
3. Линковать `liblcdhat.so` и выставить runtime search path/`LD_LIBRARY_PATH`.
4. Сделать безопасную Rust-обертку `LcdHat` с `Drop` -> `lcdhat_deinit`.
5. Проверить вызовы API: `init -> draw/clear -> present -> poll_key -> deinit`.

## Шаблон `Cargo.toml`

```toml
[dependencies]
libc = "0.2"

[build-dependencies]
cmake = "0.1"
bindgen = "0.69"
```

## Шаблон `build.rs`

```rust
use std::{env, path::PathBuf};

fn main() {
    // Позволяет выбрать backend: DEV | WIRINGPI | BCM2835
    let backend = env::var("LCDHAT_BACKEND").unwrap_or_else(|_| "DEV".to_string());

    let dst = cmake::Config::new("Display/lcd_display/1.44inch-LCD-HAT-Code/RaspberryPi/c")
        .define("LCDHAT_BACKEND", &backend)
        .build();

    let lib_dir = dst.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=lcdhat");

    // Linux runtime loading
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());

    let header = "Display/lcd_display/1.44inch-LCD-HAT-Code/RaspberryPi/c/include/lcdhat/lcdhat.h";
    let bindings = bindgen::Builder::default()
        .header(header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_type("lcdhat_.*")
        .allowlist_function("lcdhat_.*")
        .allowlist_var("LCDHAT_.*")
        .generate()
        .expect("Failed to generate lcdhat bindings");

    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR missing"));
    bindings
        .write_to_file(out.join("lcdhat_bindings.rs"))
        .expect("Failed to write bindings");
}
```

Если библиотека лежит рядом с Rust workspace, подстрой путь в `cmake::Config::new(...)` и `header`.

## Минимальная безопасная обертка (паттерн)

```rust
use std::{ffi::CString, ptr::NonNull};

pub struct LcdHat {
    raw: NonNull<lcdhat_ctx_t>,
}

impl LcdHat {
    pub fn new() -> Result<Self, lcdhat_status_t> {
        unsafe {
            let mut cfg = lcdhat_config_default();
            let mut out = std::ptr::null_mut();
            let st = lcdhat_init(&mut out, &cfg);
            if st != lcdhat_status_t_LCDHAT_OK {
                return Err(st);
            }
            Ok(Self { raw: NonNull::new(out).expect("lcdhat_init returned null") })
        }
    }

    pub fn clear_present(&mut self, color: u16) -> Result<(), lcdhat_status_t> {
        unsafe {
            let st = lcdhat_clear(self.raw.as_ptr(), color);
            if st != lcdhat_status_t_LCDHAT_OK {
                return Err(st);
            }
            let st = lcdhat_present(self.raw.as_ptr());
            if st != lcdhat_status_t_LCDHAT_OK {
                return Err(st);
            }
            Ok(())
        }
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str) -> Result<(), lcdhat_status_t> {
        let c = CString::new(text).map_err(|_| lcdhat_status_t_LCDHAT_ERR_ARG)?;
        unsafe {
            let st = lcdhat_draw_text(
                self.raw.as_ptr(),
                x,
                y,
                c.as_ptr(),
                lcdhat_font_t_LCDHAT_FONT_12,
                lcdhat_color_t_LCDHAT_COLOR_BLACK as u16,
                lcdhat_color_t_LCDHAT_COLOR_WHITE as u16,
            );
            if st != lcdhat_status_t_LCDHAT_OK {
                return Err(st);
            }
            Ok(())
        }
    }
}

impl Drop for LcdHat {
    fn drop(&mut self) {
        unsafe { lcdhat_deinit(self.raw.as_ptr()) }
    }
}
```

## Правила корректного использования из Rust

- Не создавать более одного `LcdHat` одновременно.
- Не вызывать API после `Drop`.
- Для строк всегда использовать `CString` (без `\0` внутри).
- Проверять каждый `lcdhat_status_t`; текст ошибки получать через `lcdhat_status_str`.
- Для частичного обновления помнить, что `(x1, y1)` не включаются в регион.
- В цикле ввода при отсутствии события делать `lcdhat_sleep_ms(10)` для снижения нагрузки CPU.

## Backend и окружение Raspberry Pi

- `DEV`: основной вариант; требует `lgpio` на целевом устройстве.
- `WIRINGPI`: legacy-сценарии с `wiringPi`.
- `BCM2835`: вариант с `bcm2835`.

Для запуска бинаря с `liblcdhat.so`:

- или настроить `rpath` (предпочтительно);
- или запускать с `LD_LIBRARY_PATH=<path-to-lcdhat-lib>`.

## Что делать при типичных ошибках

- `LCDHAT_ERR_HW_INIT`: backend-библиотека не установлена или нет доступа к GPIO/SPI.
- `LCDHAT_ERR_STATE` на `init`: уже есть активный контекст, закрой предыдущий.
- `LCDHAT_ERR_ARG`: выход за границы экрана, неверный `rotation`, `backlight > 1024`, `NULL` аргументы.

## Чеклист для задач агента

1. Найти текущий Rust crate и способ сборки (`build.rs` уже есть или нет).
2. Подключить сборку C библиотеки через CMake с нужным backend.
3. Настроить bindings (`bindgen`) только по `lcdhat_*` и `LCDHAT_*`.
4. Добавить безопасный wrapper с `Drop` и `Result`-ориентированным API.
5. Добавить минимальный пример: `init -> draw_text -> present -> poll_key loop`.
6. Проверить запуск на Raspberry Pi (или предупредить о stub-режиме без `lgpio`).
