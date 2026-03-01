#include "lcdhat_internal.h"

sFONT *lcdhat_resolve_font(lcdhat_font_t font)
{
    switch (font) {
    case LCDHAT_FONT_8:
        return &Font8;
    case LCDHAT_FONT_12:
        return &Font12;
    case LCDHAT_FONT_16:
        return &Font16;
    case LCDHAT_FONT_20:
        return &Font20;
    case LCDHAT_FONT_24:
        return &Font24;
    default:
        return NULL;
    }
}
