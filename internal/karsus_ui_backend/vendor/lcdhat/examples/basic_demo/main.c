#include <signal.h>
#include <stdio.h>

#include "lcdhat/lcdhat.h"

static volatile sig_atomic_t g_run = 1;

static void on_sigint(int signo)
{
    (void)signo;
    g_run = 0;
}

static void draw_scene(lcdhat_ctx_t *ctx, uint16_t marker_x, uint16_t marker_y, uint16_t marker_color)
{
    lcdhat_clear(ctx, LCDHAT_COLOR_WHITE);
    lcdhat_draw_text(ctx, 10, 4, "lcdhat demo", LCDHAT_FONT_16,
                     LCDHAT_COLOR_BLACK, LCDHAT_COLOR_WHITE);

    lcdhat_draw_line(ctx, 4, 24, 124, 24, LCDHAT_COLOR_BLUE, 2);
    lcdhat_draw_rect(ctx, 8, 30, 62, 76, LCDHAT_COLOR_RED, 2, 0);
    lcdhat_draw_circle(ctx, 96, 52, 20, LCDHAT_COLOR_GREEN, 2, 0);
    lcdhat_draw_rect(ctx, 4, 72, 124, 126, LCDHAT_COLOR_CYAN, 1, 1);
    lcdhat_draw_text(ctx, 8, 74, "U/D/L/R: MOVE", LCDHAT_FONT_12,
                     LCDHAT_COLOR_BLACK, LCDHAT_COLOR_CYAN);
    lcdhat_draw_text(ctx, 8, 91, "P: COLOR  K3: BL", LCDHAT_FONT_12,
                     LCDHAT_COLOR_BLACK, LCDHAT_COLOR_CYAN);
    lcdhat_draw_text(ctx, 8, 108, "K1: RESET K2: CLR", LCDHAT_FONT_12,
                     LCDHAT_COLOR_BLACK, LCDHAT_COLOR_CYAN);

    lcdhat_draw_circle(ctx, marker_x, marker_y, 6, marker_color, 1, 1);
    lcdhat_present(ctx);
}

int main(void)
{
    lcdhat_ctx_t *ctx = NULL;
    lcdhat_config_t cfg = lcdhat_config_default();
    lcdhat_key_event_t event;
    uint8_t has_event = 0;
    uint16_t marker_x = 64;
    uint16_t marker_y = 64;
    uint16_t marker_color = LCDHAT_COLOR_MAGENTA;
    uint8_t bright_step = 1;

    signal(SIGINT, on_sigint);

    if (lcdhat_init(&ctx, &cfg) != LCDHAT_OK) {
        fprintf(stderr, "lcdhat_init failed\n");
        return 1;
    }

    draw_scene(ctx, marker_x, marker_y, marker_color);

    while (g_run) {
        if (lcdhat_poll_key(ctx, &event, &has_event) != LCDHAT_OK) {
            break;
        }

        if (!has_event) {
            lcdhat_sleep_ms(10);
            continue;
        }

        switch (event.key) {
        case LCDHAT_KEY_UP:
            if (marker_y > 36) marker_y -= 3;
            break;
        case LCDHAT_KEY_DOWN:
            if (marker_y < 68) marker_y += 3;
            break;
        case LCDHAT_KEY_LEFT:
            if (marker_x > 12) marker_x -= 3;
            break;
        case LCDHAT_KEY_RIGHT:
            if (marker_x < 116) marker_x += 3;
            break;
        case LCDHAT_KEY_PRESS:
            marker_color = (marker_color == LCDHAT_COLOR_MAGENTA)
                               ? LCDHAT_COLOR_BLUE
                               : LCDHAT_COLOR_MAGENTA;
            break;
        case LCDHAT_KEY_K1:
            draw_scene(ctx, marker_x, marker_y, marker_color);
            continue;
        case LCDHAT_KEY_K2:
            lcdhat_clear(ctx, LCDHAT_COLOR_WHITE);
            lcdhat_present(ctx);
            continue;
        case LCDHAT_KEY_K3:
            bright_step = (uint8_t)((bright_step + 1) % 4);
            if (bright_step == 0) bright_step = 1;
            lcdhat_set_backlight(ctx, (uint16_t)(bright_step * 256));
            break;
        default:
            break;
        }

        draw_scene(ctx, marker_x, marker_y, marker_color);
    }

    lcdhat_deinit(ctx);
    return 0;
}
