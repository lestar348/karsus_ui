mod error;
mod types;

pub use error::{Error, Result};
pub use types::{Config, Font, Key, KeyEvent, LCD_HEIGHT, LCD_WIDTH, MAX_BACKLIGHT, color};

use lcdhat_sys as sys;
#[cfg(target_os = "linux")]
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

static INSTANCE_ACTIVE: AtomicBool = AtomicBool::new(false);

pub struct LcdHat {
    raw: NonNull<sys::lcdhat_ctx_t>,
    cfg: Config,
    _guard: InstanceGuard,
}

struct InstanceGuard;

impl InstanceGuard {
    fn acquire() -> Result<Self> {
        if INSTANCE_ACTIVE
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return Err(Error::AlreadyInitialized);
        }
        Ok(Self)
    }
}

impl Drop for InstanceGuard {
    fn drop(&mut self) {
        INSTANCE_ACTIVE.store(false, Ordering::Release);
    }
}

impl LcdHat {
    pub fn new(config: Config) -> Result<Self> {
        if !cfg!(target_os = "linux") {
            return Err(Error::UnsupportedPlatform);
        }

        let cfg = config.validate()?;
        let guard = InstanceGuard::acquire()?;

        let mut raw = std::ptr::null_mut();
        let ffi_cfg = cfg.to_ffi();
        let status = unsafe {
            // Safe: pointers are valid and lifetimes cover this call.
            sys::lcdhat_init(&mut raw, &ffi_cfg)
        };

        ensure_ok(status)?;

        let raw = NonNull::new(raw).ok_or(Error::Status {
            status: sys::lcdhat_status_t::LCDHAT_ERR_ALLOC,
            message: "lcdhat_init returned null context".to_string(),
        })?;

        Ok(Self {
            raw,
            cfg,
            _guard: guard,
        })
    }

    pub fn clear(&mut self, color: u16) -> Result<()> {
        let status = unsafe { sys::lcdhat_clear(self.raw.as_ptr(), color) };
        ensure_ok(status)
    }

    pub fn present(&mut self) -> Result<()> {
        let status = unsafe { sys::lcdhat_present(self.raw.as_ptr()) };
        ensure_ok(status)
    }

    pub fn present_region(&mut self, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<()> {
        validate_region_bounds(self.cfg.width, self.cfg.height, x0, y0, x1, y1)?;
        let status = unsafe { sys::lcdhat_present_region(self.raw.as_ptr(), x0, y0, x1, y1) };
        ensure_ok(status)
    }

    pub fn draw_pixel(&mut self, x: u16, y: u16, color: u16) -> Result<()> {
        let status = unsafe { sys::lcdhat_draw_pixel(self.raw.as_ptr(), x, y, color) };
        ensure_ok(status)
    }

    pub fn draw_line(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        color: u16,
        width: u8,
    ) -> Result<()> {
        let status =
            unsafe { sys::lcdhat_draw_line(self.raw.as_ptr(), x0, y0, x1, y1, color, width) };
        ensure_ok(status)
    }

    pub fn draw_rect(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
        color: u16,
        line_width: u8,
        filled: bool,
    ) -> Result<()> {
        let status = unsafe {
            sys::lcdhat_draw_rect(
                self.raw.as_ptr(),
                x0,
                y0,
                x1,
                y1,
                color,
                line_width,
                u8::from(filled),
            )
        };
        ensure_ok(status)
    }

    pub fn draw_text(
        &mut self,
        x: u16,
        y: u16,
        text: &str,
        font: Font,
        fg: u16,
        bg: u16,
    ) -> Result<()> {
        let c_text = to_cstring(text)?;
        let status = unsafe {
            sys::lcdhat_draw_text(
                self.raw.as_ptr(),
                x,
                y,
                c_text.as_ptr(),
                font.to_ffi(),
                fg,
                bg,
            )
        };
        ensure_ok(status)
    }

    pub fn set_backlight(&mut self, value: u16) -> Result<()> {
        if value > MAX_BACKLIGHT {
            return Err(Error::InvalidConfig("backlight must be in range 0..=1024"));
        }
        let status = unsafe { sys::lcdhat_set_backlight(self.raw.as_ptr(), value) };
        ensure_ok(status)
    }

    pub fn poll_key(&mut self) -> Result<Option<KeyEvent>> {
        let mut event = sys::lcdhat_key_event_t {
            key: sys::lcdhat_key_t::LCDHAT_KEY_UP,
            pressed: 0,
        };
        let mut has_event: u8 = 0;

        let status = unsafe { sys::lcdhat_poll_key(self.raw.as_ptr(), &mut event, &mut has_event) };
        ensure_ok(status)?;

        if has_event == 0 {
            return Ok(None);
        }

        let key = Key::from_ffi(event.key).ok_or(Error::UnknownKey(event.key as u32))?;
        Ok(Some(KeyEvent {
            key,
            pressed: event.pressed != 0,
        }))
    }

    pub fn sleep_ms(ms: u32) {
        unsafe { sys::lcdhat_sleep_ms(ms) }
    }
}

impl Drop for LcdHat {
    fn drop(&mut self) {
        unsafe {
            sys::lcdhat_deinit(self.raw.as_ptr());
        }
    }
}

fn ensure_ok(status: sys::lcdhat_status_t) -> Result<()> {
    if status == sys::lcdhat_status_t::LCDHAT_OK {
        return Ok(());
    }

    if status == sys::lcdhat_status_t::LCDHAT_ERR_STATE {
        return Err(Error::AlreadyInitialized);
    }

    Err(Error::Status {
        status,
        message: status_message(status),
    })
}

fn status_message(status: sys::lcdhat_status_t) -> String {
    #[cfg(target_os = "linux")]
    {
        let ptr = unsafe { sys::lcdhat_status_str(status) };
        if ptr.is_null() {
            return "unknown status".to_string();
        }
        return unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
    }

    #[cfg(not(target_os = "linux"))]
    {
        format!("{status:?}")
    }
}

fn to_cstring(text: &str) -> Result<CString> {
    CString::new(text).map_err(|_| Error::InvalidString)
}

fn validate_region_bounds(
    width: u16,
    height: u16,
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
) -> Result<()> {
    if x0 >= x1 || y0 >= y1 {
        return Err(Error::InvalidConfig(
            "present_region requires x0 < x1 and y0 < y1",
        ));
    }
    if x1 > width || y1 > height {
        return Err(Error::InvalidConfig(
            "present_region bounds exceed display dimensions",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_invalid_resolution() {
        let cfg = Config {
            width: 127,
            ..Config::default()
        };
        assert!(matches!(cfg.validate(), Err(Error::InvalidConfig(_))));
    }

    #[test]
    fn reject_invalid_rotation() {
        let cfg = Config {
            rotation: 45,
            ..Config::default()
        };
        assert!(matches!(cfg.validate(), Err(Error::InvalidConfig(_))));
    }

    #[test]
    fn reject_invalid_backlight() {
        let cfg = Config {
            backlight: 1025,
            ..Config::default()
        };
        assert!(matches!(cfg.validate(), Err(Error::InvalidConfig(_))));
    }

    #[test]
    fn cstring_rejects_embedded_nul() {
        let err = to_cstring("abc\0def").unwrap_err();
        assert!(matches!(err, Error::InvalidString));
    }

    #[test]
    fn instance_guard_is_singleton() {
        let g1 = InstanceGuard::acquire().expect("first acquire should succeed");
        let g2 = InstanceGuard::acquire();
        assert!(matches!(g2, Err(Error::AlreadyInitialized)));
        drop(g1);
        let g3 = InstanceGuard::acquire();
        assert!(g3.is_ok());
    }

    #[test]
    fn present_region_uses_exclusive_bounds() {
        assert!(validate_region_bounds(128, 128, 0, 0, 128, 128).is_ok());
        assert!(matches!(
            validate_region_bounds(128, 128, 10, 10, 10, 20),
            Err(Error::InvalidConfig(_))
        ));
        assert!(matches!(
            validate_region_bounds(128, 128, 10, 10, 129, 20),
            Err(Error::InvalidConfig(_))
        ));
    }

    #[test]
    fn status_mapping_non_ok() {
        let err = ensure_ok(sys::lcdhat_status_t::LCDHAT_ERR_ARG).unwrap_err();
        match err {
            Error::Status { status, .. } => {
                assert_eq!(status, sys::lcdhat_status_t::LCDHAT_ERR_ARG)
            }
            _ => panic!("expected status error"),
        }
    }
}
