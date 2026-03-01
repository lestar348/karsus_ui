/* automatically copied when bindgen is disabled or target is non-linux */

pub type lcdhat_ctx_t = ::core::ffi::c_void;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum lcdhat_status_t {
    LCDHAT_OK = 0,
    LCDHAT_ERR_ARG = 1,
    LCDHAT_ERR_ALLOC = 2,
    LCDHAT_ERR_HW_INIT = 3,
    LCDHAT_ERR_STATE = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum lcdhat_color_t {
    LCDHAT_COLOR_WHITE = 0xFFFF,
    LCDHAT_COLOR_BLACK = 0x0000,
    LCDHAT_COLOR_BLUE = 0x001F,
    LCDHAT_COLOR_RED = 0xF800,
    LCDHAT_COLOR_MAGENTA = 0xF81F,
    LCDHAT_COLOR_GREEN = 0x07E0,
    LCDHAT_COLOR_CYAN = 0x7FFF,
    LCDHAT_COLOR_YELLOW = 0xFFE0,
    LCDHAT_COLOR_GRAY = 0x8430,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum lcdhat_font_t {
    LCDHAT_FONT_8 = 0,
    LCDHAT_FONT_12 = 1,
    LCDHAT_FONT_16 = 2,
    LCDHAT_FONT_20 = 3,
    LCDHAT_FONT_24 = 4,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum lcdhat_key_t {
    LCDHAT_KEY_UP = 0,
    LCDHAT_KEY_DOWN = 1,
    LCDHAT_KEY_LEFT = 2,
    LCDHAT_KEY_RIGHT = 3,
    LCDHAT_KEY_PRESS = 4,
    LCDHAT_KEY_K1 = 5,
    LCDHAT_KEY_K2 = 6,
    LCDHAT_KEY_K3 = 7,
    LCDHAT_KEY_COUNT = 8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lcdhat_config_t {
    pub width: u16,
    pub height: u16,
    pub rotation: u16,
    pub background: u16,
    pub foreground: u16,
    pub backlight: u16,
    pub poll_debounce_ms: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lcdhat_key_event_t {
    pub key: lcdhat_key_t,
    pub pressed: u8,
}

unsafe extern "C" {
    pub fn lcdhat_config_default() -> lcdhat_config_t;
    pub fn lcdhat_init(out: *mut *mut lcdhat_ctx_t, cfg: *const lcdhat_config_t) -> lcdhat_status_t;
    pub fn lcdhat_deinit(ctx: *mut lcdhat_ctx_t);
    pub fn lcdhat_status_str(code: lcdhat_status_t) -> *const ::core::ffi::c_char;
    pub fn lcdhat_clear(ctx: *mut lcdhat_ctx_t, color: u16) -> lcdhat_status_t;
    pub fn lcdhat_present(ctx: *mut lcdhat_ctx_t) -> lcdhat_status_t;
    pub fn lcdhat_present_region(
        ctx: *mut lcdhat_ctx_t,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
    ) -> lcdhat_status_t;
    pub fn lcdhat_framebuffer(ctx: *mut lcdhat_ctx_t) -> *mut u16;
    pub fn lcdhat_draw_pixel(ctx: *mut lcdhat_ctx_t, x: u16, y: u16, color: u16) -> lcdhat_status_t;
    pub fn lcdhat_draw_line(
        ctx: *mut lcdhat_ctx_t,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        color: u16,
        width: u8,
    ) -> lcdhat_status_t;
    pub fn lcdhat_draw_rect(
        ctx: *mut lcdhat_ctx_t,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        color: u16,
        line_width: u8,
        filled: u8,
    ) -> lcdhat_status_t;
    pub fn lcdhat_draw_circle(
        ctx: *mut lcdhat_ctx_t,
        cx: u16,
        cy: u16,
        radius: u16,
        color: u16,
        line_width: u8,
        filled: u8,
    ) -> lcdhat_status_t;
    pub fn lcdhat_draw_text(
        ctx: *mut lcdhat_ctx_t,
        x: u16,
        y: u16,
        text: *const ::core::ffi::c_char,
        font: lcdhat_font_t,
        fg: u16,
        bg: u16,
    ) -> lcdhat_status_t;
    pub fn lcdhat_poll_key(
        ctx: *mut lcdhat_ctx_t,
        event: *mut lcdhat_key_event_t,
        has_event: *mut u8,
    ) -> lcdhat_status_t;
    pub fn lcdhat_set_backlight(ctx: *mut lcdhat_ctx_t, value: u16) -> lcdhat_status_t;
    pub fn lcdhat_sleep_ms(ms: u32);
}
