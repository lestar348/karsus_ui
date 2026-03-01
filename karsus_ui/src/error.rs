use std::fmt;

#[derive(Debug)]
pub enum UiError {
    Backend(karsus_ui_backend::Error),
    NoPages,
    InvalidLayout(&'static str),
    DuplicateWidgetId(u32),
    FocusedWidgetNotFound(u32),
}

pub type UiResult<T> = Result<T, UiError>;

impl fmt::Display for UiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UiError::Backend(err) => write!(f, "backend error: {err}"),
            UiError::NoPages => write!(f, "app has no pages in navigation stack"),
            UiError::InvalidLayout(msg) => write!(f, "invalid layout: {msg}"),
            UiError::DuplicateWidgetId(id) => write!(f, "duplicate widget id in page view: {id}"),
            UiError::FocusedWidgetNotFound(id) => {
                write!(f, "focused widget id was not found in current layout: {id}")
            }
        }
    }
}

impl std::error::Error for UiError {}

impl From<karsus_ui_backend::Error> for UiError {
    fn from(value: karsus_ui_backend::Error) -> Self {
        UiError::Backend(value)
    }
}
