use crate::error::{Error, Result};
use lcdhat_sys as sys;

pub const LCD_WIDTH: u16 = 128;
pub const LCD_HEIGHT: u16 = 128;
pub const MAX_BACKLIGHT: u16 = 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config {
    pub width: u16,
    pub height: u16,
    pub rotation: u16,
    pub background: u16,
    pub foreground: u16,
    pub backlight: u16,
    pub poll_debounce_ms: u32,
}

impl Default for Config {
    fn default() -> Self {
        #[cfg(target_os = "linux")]
        {
            // Safe: plain value-returning function with no side effects on context lifecycle.
            let cfg = unsafe { sys::lcdhat_config_default() };
            Self {
                width: cfg.width,
                height: cfg.height,
                rotation: cfg.rotation,
                background: cfg.background,
                foreground: cfg.foreground,
                backlight: cfg.backlight,
                poll_debounce_ms: cfg.poll_debounce_ms,
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            Self {
                width: LCD_WIDTH,
                height: LCD_HEIGHT,
                rotation: 0,
                background: color::WHITE,
                foreground: color::BLACK,
                backlight: MAX_BACKLIGHT,
                poll_debounce_ms: 120,
            }
        }
    }
}

impl Config {
    pub fn validate(self) -> Result<Self> {
        if self.width != LCD_WIDTH || self.height != LCD_HEIGHT {
            return Err(Error::InvalidConfig("width/height must be exactly 128x128"));
        }
        if !matches!(self.rotation, 0 | 90 | 180 | 270) {
            return Err(Error::InvalidConfig("rotation must be one of 0/90/180/270"));
        }
        if self.backlight > MAX_BACKLIGHT {
            return Err(Error::InvalidConfig("backlight must be in range 0..=1024"));
        }
        Ok(self)
    }

    pub(crate) fn to_ffi(self) -> sys::lcdhat_config_t {
        sys::lcdhat_config_t {
            width: self.width,
            height: self.height,
            rotation: self.rotation,
            background: self.background,
            foreground: self.foreground,
            backlight: self.backlight,
            poll_debounce_ms: self.poll_debounce_ms,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Font {
    Font8,
    Font12,
    Font16,
    Font20,
    Font24,
}

impl Font {
    pub(crate) fn to_ffi(self) -> sys::lcdhat_font_t {
        match self {
            Font::Font8 => sys::lcdhat_font_t::LCDHAT_FONT_8,
            Font::Font12 => sys::lcdhat_font_t::LCDHAT_FONT_12,
            Font::Font16 => sys::lcdhat_font_t::LCDHAT_FONT_16,
            Font::Font20 => sys::lcdhat_font_t::LCDHAT_FONT_20,
            Font::Font24 => sys::lcdhat_font_t::LCDHAT_FONT_24,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Press,
    K1,
    K2,
    K3,
}

impl Key {
    pub(crate) fn from_ffi(key: sys::lcdhat_key_t) -> Option<Self> {
        match key {
            sys::lcdhat_key_t::LCDHAT_KEY_UP => Some(Key::Up),
            sys::lcdhat_key_t::LCDHAT_KEY_DOWN => Some(Key::Down),
            sys::lcdhat_key_t::LCDHAT_KEY_LEFT => Some(Key::Left),
            sys::lcdhat_key_t::LCDHAT_KEY_RIGHT => Some(Key::Right),
            sys::lcdhat_key_t::LCDHAT_KEY_PRESS => Some(Key::Press),
            sys::lcdhat_key_t::LCDHAT_KEY_K1 => Some(Key::K1),
            sys::lcdhat_key_t::LCDHAT_KEY_K2 => Some(Key::K2),
            sys::lcdhat_key_t::LCDHAT_KEY_K3 => Some(Key::K3),
            sys::lcdhat_key_t::LCDHAT_KEY_COUNT => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub pressed: bool,
}

pub mod color {
    use lcdhat_sys as sys;

    pub const WHITE: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_WHITE as u16;
    pub const BLACK: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_BLACK as u16;
    pub const BLUE: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_BLUE as u16;
    pub const RED: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_RED as u16;
    pub const MAGENTA: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_MAGENTA as u16;
    pub const GREEN: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_GREEN as u16;
    pub const CYAN: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_CYAN as u16;
    pub const YELLOW: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_YELLOW as u16;
    pub const GRAY: u16 = sys::lcdhat_color_t::LCDHAT_COLOR_GRAY as u16;
}
