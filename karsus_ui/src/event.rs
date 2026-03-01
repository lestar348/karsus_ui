use crate::widget::{ButtonActionId, WidgetId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiEvent {
    Up,
    Down,
    Left,
    Right,
    Press {
        focused: WidgetId,
        action: Option<ButtonActionId>,
    },
    K1,
    K2,
    K3,
}
