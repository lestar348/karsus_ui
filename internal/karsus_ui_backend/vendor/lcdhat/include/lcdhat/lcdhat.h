#ifndef LCDHAT_LCDHAT_H
#define LCDHAT_LCDHAT_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct lcdhat_ctx lcdhat_ctx_t;

typedef enum {
    LCDHAT_OK = 0,
    LCDHAT_ERR_ARG,
    LCDHAT_ERR_ALLOC,
    LCDHAT_ERR_HW_INIT,
    LCDHAT_ERR_STATE,
} lcdhat_status_t;

typedef enum {
    LCDHAT_COLOR_WHITE = 0xFFFF,
    LCDHAT_COLOR_BLACK = 0x0000,
    LCDHAT_COLOR_BLUE = 0x001F,
    LCDHAT_COLOR_RED = 0xF800,
    LCDHAT_COLOR_MAGENTA = 0xF81F,
    LCDHAT_COLOR_GREEN = 0x07E0,
    LCDHAT_COLOR_CYAN = 0x7FFF,
    LCDHAT_COLOR_YELLOW = 0xFFE0,
    LCDHAT_COLOR_GRAY = 0x8430,
} lcdhat_color_t;

typedef enum {
    LCDHAT_FONT_8 = 0,
    LCDHAT_FONT_12,
    LCDHAT_FONT_16,
    LCDHAT_FONT_20,
    LCDHAT_FONT_24,
} lcdhat_font_t;

typedef enum {
    LCDHAT_KEY_UP = 0,
    LCDHAT_KEY_DOWN,
    LCDHAT_KEY_LEFT,
    LCDHAT_KEY_RIGHT,
    LCDHAT_KEY_PRESS,
    LCDHAT_KEY_K1,
    LCDHAT_KEY_K2,
    LCDHAT_KEY_K3,
    LCDHAT_KEY_COUNT,
} lcdhat_key_t;

typedef struct {
    uint16_t width;
    uint16_t height;
    uint16_t rotation;
    uint16_t background;
    uint16_t foreground;
    uint16_t backlight;
    uint32_t poll_debounce_ms;
} lcdhat_config_t;

typedef struct {
    lcdhat_key_t key;
    uint8_t pressed;
} lcdhat_key_event_t;

lcdhat_config_t lcdhat_config_default(void);

lcdhat_status_t lcdhat_init(lcdhat_ctx_t **out, const lcdhat_config_t *cfg);
void lcdhat_deinit(lcdhat_ctx_t *ctx);

const char *lcdhat_status_str(lcdhat_status_t code);

lcdhat_status_t lcdhat_clear(lcdhat_ctx_t *ctx, uint16_t color);
lcdhat_status_t lcdhat_present(lcdhat_ctx_t *ctx);

/* x1/y1 are exclusive bounds. */
lcdhat_status_t lcdhat_present_region(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0,
                                      uint16_t x1, uint16_t y1);

uint16_t *lcdhat_framebuffer(lcdhat_ctx_t *ctx);

lcdhat_status_t lcdhat_draw_pixel(lcdhat_ctx_t *ctx, uint16_t x, uint16_t y, uint16_t color);
lcdhat_status_t lcdhat_draw_line(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0,
                                 uint16_t x1, uint16_t y1, uint16_t color, uint8_t width);
lcdhat_status_t lcdhat_draw_rect(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0,
                                 uint16_t x1, uint16_t y1, uint16_t color,
                                 uint8_t line_width, uint8_t filled);
lcdhat_status_t lcdhat_draw_circle(lcdhat_ctx_t *ctx, uint16_t cx, uint16_t cy,
                                   uint16_t radius, uint16_t color,
                                   uint8_t line_width, uint8_t filled);
lcdhat_status_t lcdhat_draw_text(lcdhat_ctx_t *ctx, uint16_t x, uint16_t y,
                                 const char *text, lcdhat_font_t font,
                                 uint16_t fg, uint16_t bg);

lcdhat_status_t lcdhat_poll_key(lcdhat_ctx_t *ctx, lcdhat_key_event_t *event, uint8_t *has_event);
lcdhat_status_t lcdhat_set_backlight(lcdhat_ctx_t *ctx, uint16_t value);

void lcdhat_sleep_ms(uint32_t ms);

/* v1 limitation: only one active context at a time due to backend globals. */

#ifdef __cplusplus
}
#endif

#endif
