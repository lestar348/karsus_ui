# LCDHAT Public API Reference

Источник: `include/lcdhat/lcdhat.h`.

## Непрозрачный контекст

- `lcdhat_ctx_t`: opaque-тип контекста устройства. Создается в `lcdhat_init`, освобождается в `lcdhat_deinit`.

## Коды статуса `lcdhat_status_t`

- `LCDHAT_OK = 0`: успешно.
- `LCDHAT_ERR_ARG`: неверный аргумент.
- `LCDHAT_ERR_ALLOC`: ошибка выделения памяти.
- `LCDHAT_ERR_HW_INIT`: ошибка инициализации hardware backend.
- `LCDHAT_ERR_STATE`: недопустимое состояние библиотеки.

## Цвета `lcdhat_color_t` (RGB565)

- `LCDHAT_COLOR_WHITE = 0xFFFF`
- `LCDHAT_COLOR_BLACK = 0x0000`
- `LCDHAT_COLOR_BLUE = 0x001F`
- `LCDHAT_COLOR_RED = 0xF800`
- `LCDHAT_COLOR_MAGENTA = 0xF81F`
- `LCDHAT_COLOR_GREEN = 0x07E0`
- `LCDHAT_COLOR_CYAN = 0x7FFF`
- `LCDHAT_COLOR_YELLOW = 0xFFE0`
- `LCDHAT_COLOR_GRAY = 0x8430`

## Шрифты `lcdhat_font_t`

- `LCDHAT_FONT_8`
- `LCDHAT_FONT_12`
- `LCDHAT_FONT_16`
- `LCDHAT_FONT_20`
- `LCDHAT_FONT_24`

## Кнопки `lcdhat_key_t`

- `LCDHAT_KEY_UP`
- `LCDHAT_KEY_DOWN`
- `LCDHAT_KEY_LEFT`
- `LCDHAT_KEY_RIGHT`
- `LCDHAT_KEY_PRESS`
- `LCDHAT_KEY_K1`
- `LCDHAT_KEY_K2`
- `LCDHAT_KEY_K3`
- `LCDHAT_KEY_COUNT`: количество ключей (служебный enum-элемент).

## Структуры конфигурации и событий

- `lcdhat_config_t`:
  - `width`, `height`: размер экрана; поддерживается только штатный размер панели.
  - `rotation`: поворот (разрешено `0/90/180/270`).
  - `background`, `foreground`: цвета по умолчанию.
  - `backlight`: уровень подсветки (`0..=1024`).
  - `poll_debounce_ms`: debounce для `lcdhat_poll_key`.
- `lcdhat_key_event_t`:
  - `key`: какая кнопка.
  - `pressed`: признак нажатия (в текущем API используется значение `1` на событии фронта).

## Функции API

- `lcdhat_config_t lcdhat_config_default(void)`
  - Возвращает дефолтную конфигурацию.
  - Используй как базу перед кастомизацией параметров.

- `lcdhat_status_t lcdhat_init(lcdhat_ctx_t **out, const lcdhat_config_t *cfg)`
  - Инициализирует backend, создает контекст и framebuffer.
  - `out` обязателен, `cfg == NULL` допустим (взять defaults).
  - Ошибки: `LCDHAT_ERR_ARG`, `LCDHAT_ERR_ALLOC`, `LCDHAT_ERR_HW_INIT`, `LCDHAT_ERR_STATE`.

- `void lcdhat_deinit(lcdhat_ctx_t *ctx)`
  - Освобождает framebuffer/контекст и завершает backend.
  - Безопасно вызывать только для ранее успешно инициализированного `ctx`.

- `const char *lcdhat_status_str(lcdhat_status_t code)`
  - Возвращает строковое описание статуса.
  - Удобно для логов и преобразования в Rust error messages.

- `lcdhat_status_t lcdhat_clear(lcdhat_ctx_t *ctx, uint16_t color)`
  - Очищает framebuffer цветом `color`.
  - Требует `ctx != NULL`.

- `lcdhat_status_t lcdhat_present(lcdhat_ctx_t *ctx)`
  - Полное обновление дисплея из framebuffer.

- `lcdhat_status_t lcdhat_present_region(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0, uint16_t x1, uint16_t y1)`
  - Частичное обновление области.
  - Контракт: `x1/y1` эксклюзивны, валидная область `x0 < x1`, `y0 < y1`.

- `uint16_t *lcdhat_framebuffer(lcdhat_ctx_t *ctx)`
  - Возвращает pointer на внутренний framebuffer (RGB565).
  - Требует аккуратности в Rust: это raw mutable pointer без проверок границ.

- `lcdhat_status_t lcdhat_draw_pixel(lcdhat_ctx_t *ctx, uint16_t x, uint16_t y, uint16_t color)`
  - Рисует пиксель.
  - Проверяет границы координат.

- `lcdhat_status_t lcdhat_draw_line(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0, uint16_t x1, uint16_t y1, uint16_t color, uint8_t width)`
  - Рисует линию.
  - `width` мапится на внутренние dot-предустановки.

- `lcdhat_status_t lcdhat_draw_rect(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0, uint16_t x1, uint16_t y1, uint16_t color, uint8_t line_width, uint8_t filled)`
  - Рисует прямоугольник.
  - `filled != 0` -> заливка, иначе только контур.

- `lcdhat_status_t lcdhat_draw_circle(lcdhat_ctx_t *ctx, uint16_t cx, uint16_t cy, uint16_t radius, uint16_t color, uint8_t line_width, uint8_t filled)`
  - Рисует окружность.
  - `radius` должен быть `> 0`.

- `lcdhat_status_t lcdhat_draw_text(lcdhat_ctx_t *ctx, uint16_t x, uint16_t y, const char *text, lcdhat_font_t font, uint16_t fg, uint16_t bg)`
  - Рисует ASCII-строку.
  - `text` обязателен и должен быть корректной C-строкой.

- `lcdhat_status_t lcdhat_poll_key(lcdhat_ctx_t *ctx, lcdhat_key_event_t *event, uint8_t *has_event)`
  - Неблокирующий polling кнопок.
  - При событии: `*has_event = 1`, заполнен `event`.
  - Без события: `*has_event = 0`.

- `lcdhat_status_t lcdhat_set_backlight(lcdhat_ctx_t *ctx, uint16_t value)`
  - Устанавливает подсветку в диапазоне `0..=1024`.

- `void lcdhat_sleep_ms(uint32_t ms)`
  - Вспомогательная задержка (миллисекунды) через backend.
