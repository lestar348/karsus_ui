use karsus_ui_backend::Font;

pub type WidgetId = u32;
pub type ButtonActionId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonPalette {
    pub background: u16,
    pub foreground: u16,
    pub border: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonStyle {
    pub normal: ButtonPalette,
    pub focused: ButtonPalette,
    pub border_width: u8,
    pub padding: u16,
}

impl ButtonStyle {
    pub fn themed(theme: crate::Theme) -> Self {
        Self {
            normal: ButtonPalette {
                background: theme.background,
                foreground: theme.on_background,
                border: theme.primary,
            },
            focused: ButtonPalette {
                background: theme.secondary,
                foreground: theme.on_secondary,
                border: theme.primary,
            },
            border_width: 1,
            padding: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    pub text: String,
    pub color: Option<u16>,
    pub font: Font,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            color: None,
            font: Font::Font12,
        }
    }

    pub fn color(mut self, color: u16) -> Self {
        self.color = Some(color);
        self
    }

    pub fn font(mut self, font: Font) -> Self {
        self.font = font;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Button {
    pub id: WidgetId,
    pub label: String,
    pub style: Option<ButtonStyle>,
    pub on_press: Option<ButtonActionId>,
}

impl Button {
    pub fn new(id: WidgetId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            style: None,
            on_press: None,
        }
    }

    pub fn with_style(mut self, style: ButtonStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn with_theme(self, theme: crate::Theme) -> Self {
        self.with_style(ButtonStyle::themed(theme))
    }

    pub fn on_press(mut self, action: ButtonActionId) -> Self {
        self.on_press = Some(action);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub children: Vec<Widget>,
    pub spacing: u16,
}

impl Row {
    pub fn new(children: Vec<Widget>) -> Self {
        Self {
            children,
            spacing: 0,
        }
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Column {
    pub children: Vec<Widget>,
    pub spacing: u16,
}

impl Column {
    pub fn new(children: Vec<Widget>) -> Self {
        Self {
            children,
            spacing: 0,
        }
    }

    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spacing = spacing;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Widget {
    Text(Text),
    Button(Button),
    Row(Row),
    Column(Column),
}

impl Widget {
    pub fn text(text: impl Into<String>) -> Self {
        Self::Text(Text::new(text))
    }

    pub fn button(id: WidgetId, label: impl Into<String>) -> Self {
        Self::Button(Button::new(id, label))
    }

    pub fn button_with_style(id: WidgetId, label: impl Into<String>, style: ButtonStyle) -> Self {
        Self::Button(Button::new(id, label).with_style(style))
    }

    pub fn row(children: Vec<Widget>) -> Self {
        Self::Row(Row::new(children))
    }

    pub fn column(children: Vec<Widget>) -> Self {
        Self::Column(Column::new(children))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themed_style_uses_on_secondary_for_focused_foreground() {
        let theme = crate::Theme {
            background: 1,
            on_background: 2,
            primary: 3,
            on_primary: 4,
            secondary: 5,
            on_secondary: 6,
        };

        let style = ButtonStyle::themed(theme);
        assert_eq!(style.focused.foreground, 6);
        assert_eq!(style.normal.foreground, 2);
    }
}
