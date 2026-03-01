#include "lcdhat_internal.h"
#include "lcdhat_log.h"

#include <stdlib.h>
#include <string.h>
#include <time.h>

static uint8_t g_ctx_active = 0;

static uint64_t lcdhat_now_ms(void)
{
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ((uint64_t)ts.tv_sec * 1000ULL) + ((uint64_t)ts.tv_nsec / 1000000ULL);
}

static const char *lcdhat_key_name(lcdhat_key_t key)
{
    switch (key) {
    case LCDHAT_KEY_UP:
        return "UP";
    case LCDHAT_KEY_DOWN:
        return "DOWN";
    case LCDHAT_KEY_LEFT:
        return "LEFT";
    case LCDHAT_KEY_RIGHT:
        return "RIGHT";
    case LCDHAT_KEY_PRESS:
        return "PRESS";
    case LCDHAT_KEY_K1:
        return "K1";
    case LCDHAT_KEY_K2:
        return "K2";
    case LCDHAT_KEY_K3:
        return "K3";
    default:
        return "UNKNOWN";
    }
}

static int valid_rotation(uint16_t rotation)
{
    return rotation == 0 || rotation == 90 || rotation == 180 || rotation == 270;
}

static lcdhat_status_t validate_ctx(lcdhat_ctx_t *ctx)
{
    if (ctx == NULL || ctx->fb == NULL) {
        lcdhat_log_error("Invalid context");
        return LCDHAT_ERR_ARG;
    }
    return LCDHAT_OK;
}

static lcdhat_status_t validate_point(lcdhat_ctx_t *ctx, uint16_t x, uint16_t y)
{
    lcdhat_status_t st = validate_ctx(ctx);
    if (st != LCDHAT_OK) {
        return st;
    }
    if (x >= ctx->cfg.width || y >= ctx->cfg.height) {
        lcdhat_log_error("Point out of bounds: x=%u y=%u (max=%ux%u)",
                         x, y, ctx->cfg.width, ctx->cfg.height);
        return LCDHAT_ERR_ARG;
    }
    return LCDHAT_OK;
}

static lcdhat_status_t validate_region(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0,
                                       uint16_t x1, uint16_t y1)
{
    lcdhat_status_t st = validate_ctx(ctx);
    if (st != LCDHAT_OK) {
        return st;
    }
    if (x0 >= x1 || y0 >= y1) {
        lcdhat_log_error("Invalid region: (%u,%u)-(%u,%u)", x0, y0, x1, y1);
        return LCDHAT_ERR_ARG;
    }
    if (x1 > ctx->cfg.width || y1 > ctx->cfg.height) {
        lcdhat_log_error("Region out of bounds: (%u,%u)-(%u,%u), max=%ux%u",
                         x0, y0, x1, y1, ctx->cfg.width, ctx->cfg.height);
        return LCDHAT_ERR_ARG;
    }
    return LCDHAT_OK;
}

static lcdhat_status_t ensure_paint_selected(lcdhat_ctx_t *ctx)
{
    lcdhat_status_t st = validate_ctx(ctx);
    if (st != LCDHAT_OK) {
        return st;
    }
    Paint_SelectImage(ctx->fb);
    return LCDHAT_OK;
}

static DOT_PIXEL to_dot_pixel(uint8_t width)
{
    switch (width) {
    case 1:
        return DOT_PIXEL_1X1;
    case 2:
        return DOT_PIXEL_2X2;
    case 3:
        return DOT_PIXEL_3X3;
    case 4:
        return DOT_PIXEL_4X4;
    case 5:
        return DOT_PIXEL_5X5;
    case 6:
        return DOT_PIXEL_6X6;
    case 7:
        return DOT_PIXEL_7X7;
    default:
        return DOT_PIXEL_8X8;
    }
}

static UBYTE read_key_state(lcdhat_key_t key)
{
    switch (key) {
    case LCDHAT_KEY_UP:
        return GET_KEY_UP == 0;
    case LCDHAT_KEY_DOWN:
        return GET_KEY_DOWN == 0;
    case LCDHAT_KEY_LEFT:
        return GET_KEY_LEFT == 0;
    case LCDHAT_KEY_RIGHT:
        return GET_KEY_RIGHT == 0;
    case LCDHAT_KEY_PRESS:
        return GET_KEY_PRESS == 0;
    case LCDHAT_KEY_K1:
        return GET_KEY1 == 0;
    case LCDHAT_KEY_K2:
        return GET_KEY2 == 0;
    case LCDHAT_KEY_K3:
        return GET_KEY3 == 0;
    default:
        return 0;
    }
}

lcdhat_config_t lcdhat_config_default(void)
{
    lcdhat_config_t cfg;
    cfg.width = LCD_WIDTH;
    cfg.height = LCD_HEIGHT;
    cfg.rotation = 0;
    cfg.background = LCDHAT_COLOR_WHITE;
    cfg.foreground = LCDHAT_COLOR_BLACK;
    cfg.backlight = 512;
    cfg.poll_debounce_ms = 30;
    return cfg;
}

lcdhat_status_t lcdhat_init(lcdhat_ctx_t **out, const lcdhat_config_t *cfg)
{
    lcdhat_ctx_t *ctx = NULL;
    lcdhat_config_t effective_cfg;
    size_t image_size;

    lcdhat_log_init();
    lcdhat_log_debug("Init requested");

    if (out == NULL) {
        lcdhat_log_error("lcdhat_init: out is NULL");
        return LCDHAT_ERR_ARG;
    }
    if (g_ctx_active) {
        lcdhat_log_error("lcdhat_init: only one active context is allowed");
        return LCDHAT_ERR_STATE;
    }

    effective_cfg = cfg ? *cfg : lcdhat_config_default();
    if (effective_cfg.width != LCD_WIDTH || effective_cfg.height != LCD_HEIGHT) {
        lcdhat_log_error("Unsupported resolution: %ux%u (expected %ux%u)",
                         effective_cfg.width, effective_cfg.height, LCD_WIDTH, LCD_HEIGHT);
        return LCDHAT_ERR_ARG;
    }
    if (!valid_rotation(effective_cfg.rotation)) {
        lcdhat_log_error("Unsupported rotation: %u (allowed: 0/90/180/270)",
                         effective_cfg.rotation);
        return LCDHAT_ERR_ARG;
    }

    ctx = (lcdhat_ctx_t *)calloc(1, sizeof(*ctx));
    if (ctx == NULL) {
        lcdhat_log_error("lcdhat_init: context allocation failed");
        return LCDHAT_ERR_ALLOC;
    }

    image_size = (size_t)effective_cfg.width * (size_t)effective_cfg.height * sizeof(uint16_t);
    ctx->fb = (uint16_t *)malloc(image_size);
    if (ctx->fb == NULL) {
        lcdhat_log_error("lcdhat_init: framebuffer allocation failed, bytes=%zu", image_size);
        free(ctx);
        return LCDHAT_ERR_ALLOC;
    }

    if (DEV_ModuleInit() != 0) {
        lcdhat_log_error("lcdhat_init: hardware module initialization failed");
        free(ctx->fb);
        free(ctx);
        return LCDHAT_ERR_HW_INIT;
    }

    LCD_1in44_Init(SCAN_DIR_DFT);
    Paint_NewImage(ctx->fb, effective_cfg.width, effective_cfg.height,
                   effective_cfg.rotation, effective_cfg.background, 16);
    Paint_Clear(effective_cfg.background);

    ctx->cfg = effective_cfg;
    if (lcdhat_set_backlight(ctx, effective_cfg.backlight) != LCDHAT_OK) {
        lcdhat_log_error("lcdhat_init: unable to apply initial backlight=%u", effective_cfg.backlight);
    }

    g_ctx_active = 1;
    *out = ctx;
    lcdhat_log_debug("Init completed: %ux%u rotation=%u bg=0x%04X fg=0x%04X debounce=%u",
                     ctx->cfg.width, ctx->cfg.height, ctx->cfg.rotation,
                     ctx->cfg.background, ctx->cfg.foreground, ctx->cfg.poll_debounce_ms);
    return LCDHAT_OK;
}

void lcdhat_deinit(lcdhat_ctx_t *ctx)
{
    if (ctx == NULL) {
        lcdhat_log_error("lcdhat_deinit: ctx is NULL");
        return;
    }

    lcdhat_log_debug("Deinit requested");

    if (ctx->fb != NULL) {
        free(ctx->fb);
        ctx->fb = NULL;
    }

    if (g_ctx_active) {
        DEV_ModuleExit();
    }

    g_ctx_active = 0;
    free(ctx);
    lcdhat_log_debug("Deinit completed");
    lcdhat_log_close();
}

const char *lcdhat_status_str(lcdhat_status_t code)
{
    switch (code) {
    case LCDHAT_OK:
        return "OK";
    case LCDHAT_ERR_ARG:
        return "Invalid argument";
    case LCDHAT_ERR_ALLOC:
        return "Allocation failed";
    case LCDHAT_ERR_HW_INIT:
        return "Hardware initialization failed";
    case LCDHAT_ERR_STATE:
        return "Invalid library state";
    default:
        return "Unknown status";
    }
}

lcdhat_status_t lcdhat_clear(lcdhat_ctx_t *ctx, uint16_t color)
{
    lcdhat_status_t st = ensure_paint_selected(ctx);
    if (st != LCDHAT_OK) {
        return st;
    }

    Paint_Clear(color);
    lcdhat_log_debug("Clear framebuffer with color=0x%04X", color);
    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_present(lcdhat_ctx_t *ctx)
{
    lcdhat_status_t st = validate_ctx(ctx);
    if (st != LCDHAT_OK) {
        return st;
    }

    LCD_1in44_Display(ctx->fb);
    lcdhat_log_debug("Full present (rerender)");
    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_present_region(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0,
                                      uint16_t x1, uint16_t y1)
{
    lcdhat_status_t st = validate_region(ctx, x0, y0, x1, y1);
    if (st != LCDHAT_OK) {
        return st;
    }

    GUI_Partial_Refresh(x0, y0, x1, y1);
    lcdhat_log_debug("Partial present (rerender): (%u,%u)-(%u,%u)", x0, y0, x1, y1);
    return LCDHAT_OK;
}

uint16_t *lcdhat_framebuffer(lcdhat_ctx_t *ctx)
{
    if (ctx == NULL) {
        lcdhat_log_error("lcdhat_framebuffer: ctx is NULL");
        return NULL;
    }
    return ctx->fb;
}

lcdhat_status_t lcdhat_draw_pixel(lcdhat_ctx_t *ctx, uint16_t x, uint16_t y, uint16_t color)
{
    lcdhat_status_t st = validate_point(ctx, x, y);
    if (st != LCDHAT_OK) {
        return st;
    }

    ensure_paint_selected(ctx);
    Paint_SetPixel(x, y, color);
    lcdhat_log_debug("Draw pixel: x=%u y=%u color=0x%04X", x, y, color);
    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_draw_line(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0,
                                 uint16_t x1, uint16_t y1, uint16_t color, uint8_t width)
{
    lcdhat_status_t st = validate_point(ctx, x0, y0);
    if (st != LCDHAT_OK) {
        return st;
    }
    st = validate_point(ctx, x1, y1);
    if (st != LCDHAT_OK) {
        return st;
    }

    ensure_paint_selected(ctx);
    Paint_DrawLine(x0, y0, x1, y1, color, to_dot_pixel(width), LINE_STYLE_SOLID);
    lcdhat_log_debug("Draw line: (%u,%u)-(%u,%u) color=0x%04X width=%u",
                     x0, y0, x1, y1, color, width);
    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_draw_rect(lcdhat_ctx_t *ctx, uint16_t x0, uint16_t y0,
                                 uint16_t x1, uint16_t y1, uint16_t color,
                                 uint8_t line_width, uint8_t filled)
{
    lcdhat_status_t st = validate_point(ctx, x0, y0);
    if (st != LCDHAT_OK) {
        return st;
    }
    st = validate_point(ctx, x1, y1);
    if (st != LCDHAT_OK) {
        return st;
    }

    ensure_paint_selected(ctx);
    Paint_DrawRectangle(x0, y0, x1, y1, color, to_dot_pixel(line_width),
                        filled ? DRAW_FILL_FULL : DRAW_FILL_EMPTY);
    lcdhat_log_debug("Draw rect: (%u,%u)-(%u,%u) color=0x%04X width=%u filled=%u",
                     x0, y0, x1, y1, color, line_width, filled);
    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_draw_circle(lcdhat_ctx_t *ctx, uint16_t cx, uint16_t cy,
                                   uint16_t radius, uint16_t color,
                                   uint8_t line_width, uint8_t filled)
{
    lcdhat_status_t st = validate_point(ctx, cx, cy);
    if (st != LCDHAT_OK) {
        return st;
    }
    if (radius == 0) {
        lcdhat_log_error("Draw circle: radius must be > 0");
        return LCDHAT_ERR_ARG;
    }

    ensure_paint_selected(ctx);
    Paint_DrawCircle(cx, cy, radius, color, to_dot_pixel(line_width),
                     filled ? DRAW_FILL_FULL : DRAW_FILL_EMPTY);
    lcdhat_log_debug("Draw circle: center=(%u,%u) radius=%u color=0x%04X width=%u filled=%u",
                     cx, cy, radius, color, line_width, filled);
    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_draw_text(lcdhat_ctx_t *ctx, uint16_t x, uint16_t y,
                                 const char *text, lcdhat_font_t font,
                                 uint16_t fg, uint16_t bg)
{
    sFONT *resolved;
    lcdhat_status_t st = validate_point(ctx, x, y);
    if (st != LCDHAT_OK) {
        return st;
    }
    if (text == NULL) {
        lcdhat_log_error("Draw text: text is NULL");
        return LCDHAT_ERR_ARG;
    }

    resolved = lcdhat_resolve_font(font);
    if (resolved == NULL) {
        lcdhat_log_error("Draw text: unsupported font enum=%d", (int)font);
        return LCDHAT_ERR_ARG;
    }

    ensure_paint_selected(ctx);
    Paint_DrawString_EN(x, y, text, resolved, fg, bg);
    lcdhat_log_debug("Draw text: x=%u y=%u font=%d fg=0x%04X bg=0x%04X text='%s'",
                     x, y, (int)font, fg, bg, text);
    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_poll_key(lcdhat_ctx_t *ctx, lcdhat_key_event_t *event, uint8_t *has_event)
{
    uint64_t now;
    int key;
    lcdhat_status_t st = validate_ctx(ctx);
    if (st != LCDHAT_OK) {
        return st;
    }
    if (event == NULL || has_event == NULL) {
        lcdhat_log_error("lcdhat_poll_key: event/has_event is NULL");
        return LCDHAT_ERR_ARG;
    }

    *has_event = 0;
    now = lcdhat_now_ms();

    for (key = 0; key < LCDHAT_KEY_COUNT; key++) {
        uint8_t pressed_now = read_key_state((lcdhat_key_t)key);
        uint8_t pressed_before = ctx->last_pressed[key];

        if (pressed_now && !pressed_before) {
            uint64_t elapsed = now - ctx->last_press_ms[key];
            if (elapsed >= ctx->cfg.poll_debounce_ms) {
                ctx->last_press_ms[key] = now;
                event->key = (lcdhat_key_t)key;
                event->pressed = 1;
                *has_event = 1;
                lcdhat_log_debug("Key detected: %s (debounce=%ums)",
                                 lcdhat_key_name(event->key), ctx->cfg.poll_debounce_ms);
            }
        }

        ctx->last_pressed[key] = pressed_now;
        if (*has_event) {
            return LCDHAT_OK;
        }
    }

    return LCDHAT_OK;
}

lcdhat_status_t lcdhat_set_backlight(lcdhat_ctx_t *ctx, uint16_t value)
{
    lcdhat_status_t st = validate_ctx(ctx);
    if (st != LCDHAT_OK) {
        return st;
    }
    if (value > 1024) {
        lcdhat_log_error("Backlight out of range: %u (expected 0..1024)", value);
        return LCDHAT_ERR_ARG;
    }

    DEV_SetBacklight(value);
    ctx->cfg.backlight = value;
    lcdhat_log_debug("Backlight set: %u", value);
    return LCDHAT_OK;
}

void lcdhat_sleep_ms(uint32_t ms)
{
    DEV_Delay_ms(ms);
}
