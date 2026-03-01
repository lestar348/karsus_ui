#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub background: u16,
    pub on_background: u16,
    pub primary: u16,
    pub on_primary: u16,
    pub secondary: u16,
    pub on_secondary: u16,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: karsus_ui_backend::color::WHITE,
            on_background: karsus_ui_backend::color::BLACK,
            primary: karsus_ui_backend::color::BLACK,
            on_primary: karsus_ui_backend::color::WHITE,
            secondary: karsus_ui_backend::color::BLUE,
            on_secondary: karsus_ui_backend::color::WHITE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastPagePolicy {
    ExitApp,
    IgnoreBack,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppConfig {
    pub theme: Theme,
    pub last_page_policy: LastPagePolicy,
    pub backend: karsus_ui_backend::Config,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            last_page_policy: LastPagePolicy::ExitApp,
            backend: karsus_ui_backend::Config::default(),
        }
    }
}
