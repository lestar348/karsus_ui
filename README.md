# karsus_ui workspace

Мини-фреймворк для UI-приложений на Raspberry Pi с Waveshare LCD HAT (GPIO), построенный поверх `karsus_ui_backend`.

## Что внутри

- `karsus_ui/` — основной UI-фреймворк (виджеты, layout, focus, страницы, runtime).
- `example/` — demo-приложение на 2 страницы.
- `internal/karsus_ui_backend/` — backend-слой для LCD HAT (FFI + safe wrapper).

## Возможности фреймворка

- Декларативные виджеты: `Text`, `Button`, `Row`, `Column`.
- Кастомизируемые кнопки (`ButtonStyle`) с визуальным focus-состоянием.
- Фокус по кнопкам с навигацией `UP/DOWN/LEFT/RIGHT` по геометрии на экране.
- Нажатие кнопки только при условии:
  - кнопка в фокусе;
  - пришло событие `LCDHAT_KEY_PRESS`.
- Навигация страниц стеком:
  - `Push`, `Pop`, `Replace`, `Exit`.
- `K1` используется как «назад» (поведение на последней странице задается в конфиге).
- `K2` и `K3` пробрасываются в `Page::on_event` для расширения.
- Полный redraw после изменений состояния/фокуса/навигации.

## Быстрый старт

### 1) Инициализация приложения

```rust
use karsus_ui::{App, AppConfig, LastPagePolicy, Theme, UiError};

fn main() -> Result<(), UiError> {
    let config = AppConfig {
        theme: Theme {
            background: karsus_ui::color::WHITE,
            primary: karsus_ui::color::BLACK,
            on_primary: karsus_ui::color::WHITE,
            secondary: karsus_ui::color::BLUE,
            on_secondary: karsus_ui::color::WHITE,
        },
        last_page_policy: LastPagePolicy::ExitApp,
        ..AppConfig::default()
    };

    let mut app = App::new(config)?;
    // app.push_page(...)
    app.run()
}
```

### 2) Определение страницы

```rust
use karsus_ui::{Button, ButtonStyle, Column, Page, PageCommand, Theme, UiEvent, Widget};

const BTN_NEXT: u32 = 1;
const ACTION_NEXT: u32 = 100;

struct HomePage;

impl Page for HomePage {
    fn view(&self) -> Widget {
        let style = ButtonStyle::themed(Theme::default());
        Widget::Column(
            Column::new(vec![
                Widget::text("Home"),
                Widget::Button(Button::new(BTN_NEXT, "Next", style).on_press(ACTION_NEXT)),
            ])
            .spacing(4),
        )
    }

    fn on_event(&mut self, event: UiEvent) -> PageCommand {
        match event {
            UiEvent::Press {
                action: Some(ACTION_NEXT),
                ..
            } => {
                // PageCommand::Push(Box::new(AnotherPage))
                PageCommand::None
            }
            UiEvent::K2 => PageCommand::None,
            UiEvent::K3 => PageCommand::None,
            _ => PageCommand::None,
        }
    }
}
```

### 3) Навигация

Навигация управляется через `PageCommand` из `on_event`:

- `PageCommand::Push(Box<dyn Page>)`
- `PageCommand::Pop`
- `PageCommand::Replace(Box<dyn Page>)`
- `PageCommand::Exit`
- `PageCommand::None`

## Конфиг приложения

`AppConfig`:

- `theme.background` — цвет фона.
- `theme.primary` — основной цвет (текст/границы).
- `theme.on_primary` — цвет контента поверх `primary`.
- `theme.secondary` — акцентный цвет (например, focused-кнопка).
- `theme.on_secondary` — цвет текста/иконок поверх `secondary` (используется для текста focused-кнопки).
- `last_page_policy`:
  - `LastPagePolicy::ExitApp` — `K1` на последней странице завершает `run()`.
  - `LastPagePolicy::IgnoreBack` — `K1` на последней странице игнорируется.
- `backend` — конфиг backend (`karsus_ui_backend::Config`).

## Команды сборки и запуска

Из корня workspace (`/Users/kirillperminov/Develop/Projects/Pet/rust/karsus_ui`):

```bash
# Проверка фреймворка
cargo test -p karsus_ui

# Сборка example
cargo build -p example

# Проверка example без запуска
cargo check -p example

# Запуск example
cargo run -p example
```

Если вы работаете на Raspberry Pi и нужна настройка backend, можно задать:

```bash
export LCDHAT_BACKEND=DEV
```

Подробности про backend и требования к окружению — в `internal/karsus_ui_backend/README.md`.

## Полезные замечания

- Экран backend фиксирован как `128x128`.
- Только события с `pressed=true` обрабатываются как действия пользователя.
- В текущей версии используется полный redraw кадра.
- `WidgetId` у кнопок должен быть уникален в пределах `view()` конкретной страницы.
