#ifndef LCDHAT_INTERNAL_H
#define LCDHAT_INTERNAL_H

#include <stddef.h>
#include <stdint.h>

#include "lcdhat/lcdhat.h"

#include "DEV_Config.h"
#include "GUI_Paint.h"
#include "LCD_1in44.h"
#include "fonts.h"

struct lcdhat_ctx {
    uint16_t *fb;
    lcdhat_config_t cfg;
    uint8_t last_pressed[LCDHAT_KEY_COUNT];
    uint64_t last_press_ms[LCDHAT_KEY_COUNT];
};

sFONT *lcdhat_resolve_font(lcdhat_font_t font);

#endif
