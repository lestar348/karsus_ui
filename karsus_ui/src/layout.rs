use crate::{ButtonActionId, ButtonStyle, UiError, UiResult, Widget, WidgetId};
use karsus_ui_backend::Font;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextLayout {
    pub rect: Rect,
    pub text: String,
    pub color: Option<u16>,
    pub font: Font,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonLayout {
    pub rect: Rect,
    pub id: WidgetId,
    pub label: String,
    pub style: ButtonStyle,
    pub action: Option<ButtonActionId>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LayoutFrame {
    pub texts: Vec<TextLayout>,
    pub buttons: Vec<ButtonLayout>,
}

impl LayoutFrame {
    pub fn button_by_id(&self, id: WidgetId) -> Option<&ButtonLayout> {
        self.buttons.iter().find(|button| button.id == id)
    }
}

pub fn build_layout(root: &Widget, bounds: Rect) -> UiResult<LayoutFrame> {
    let mut frame = LayoutFrame::default();
    let mut seen_ids = HashSet::new();
    layout_widget(root, bounds, &mut frame, &mut seen_ids)?;
    Ok(frame)
}

pub fn initial_focus(frame: &LayoutFrame) -> Option<WidgetId> {
    frame.buttons.first().map(|button| button.id)
}

pub fn move_focus(
    frame: &LayoutFrame,
    current: Option<WidgetId>,
    direction: FocusDirection,
) -> Option<WidgetId> {
    if frame.buttons.is_empty() {
        return None;
    }

    let current_id = current.or_else(|| initial_focus(frame))?;
    let current_button = frame.button_by_id(current_id)?;
    let (cx, cy) = center(current_button.rect);

    frame
        .buttons
        .iter()
        .filter(|candidate| candidate.id != current_id)
        .filter_map(|candidate| {
            let (tx, ty) = center(candidate.rect);
            let (axis, ortho) = match direction {
                FocusDirection::Up => (cy - ty, (cx - tx).abs()),
                FocusDirection::Down => (ty - cy, (cx - tx).abs()),
                FocusDirection::Left => (cx - tx, (cy - ty).abs()),
                FocusDirection::Right => (tx - cx, (cy - ty).abs()),
            };

            if axis <= 0 {
                return None;
            }

            let score = axis * 100 + ortho;
            Some((score, candidate.id))
        })
        .min_by_key(|entry| entry.0)
        .map(|entry| entry.1)
        .or(current)
}

fn layout_widget(
    widget: &Widget,
    bounds: Rect,
    frame: &mut LayoutFrame,
    seen_ids: &mut HashSet<WidgetId>,
) -> UiResult<()> {
    match widget {
        Widget::Text(text) => {
            frame.texts.push(TextLayout {
                rect: Rect {
                    x: bounds.x,
                    y: bounds.y,
                    width: bounds.width,
                    height: text_height(text.font),
                },
                text: text.text.clone(),
                color: text.color,
                font: text.font,
            });
        }
        Widget::Button(button) => {
            if !seen_ids.insert(button.id) {
                return Err(UiError::DuplicateWidgetId(button.id));
            }

            frame.buttons.push(ButtonLayout {
                rect: bounds,
                id: button.id,
                label: button.label.clone(),
                style: button.style,
                action: button.on_press,
            });
        }
        Widget::Row(row) => {
            layout_children_horizontal(&row.children, row.spacing, bounds, frame, seen_ids)?;
        }
        Widget::Column(column) => {
            layout_children_vertical(&column.children, column.spacing, bounds, frame, seen_ids)?;
        }
    }

    Ok(())
}

fn layout_children_horizontal(
    children: &[Widget],
    spacing: u16,
    bounds: Rect,
    frame: &mut LayoutFrame,
    seen_ids: &mut HashSet<WidgetId>,
) -> UiResult<()> {
    if children.is_empty() {
        return Ok(());
    }

    let child_count = children.len() as u16;
    let total_spacing = spacing.saturating_mul(child_count.saturating_sub(1));
    if total_spacing >= bounds.width {
        return Err(UiError::InvalidLayout(
            "row spacing exceeds available width",
        ));
    }

    let available = bounds.width - total_spacing;
    let cell_width = available / child_count;
    let remainder = available % child_count;

    let mut x = bounds.x;
    for (index, child) in children.iter().enumerate() {
        let extra = u16::from((index as u16) < remainder);
        let width = cell_width + extra;
        let child_rect = Rect {
            x,
            y: bounds.y,
            width,
            height: bounds.height,
        };
        layout_widget(child, child_rect, frame, seen_ids)?;
        x = x.saturating_add(width).saturating_add(spacing);
    }

    Ok(())
}

fn layout_children_vertical(
    children: &[Widget],
    spacing: u16,
    bounds: Rect,
    frame: &mut LayoutFrame,
    seen_ids: &mut HashSet<WidgetId>,
) -> UiResult<()> {
    if children.is_empty() {
        return Ok(());
    }

    let child_count = children.len() as u16;
    let total_spacing = spacing.saturating_mul(child_count.saturating_sub(1));
    if total_spacing >= bounds.height {
        return Err(UiError::InvalidLayout(
            "column spacing exceeds available height",
        ));
    }

    let available = bounds.height - total_spacing;
    let cell_height = available / child_count;
    let remainder = available % child_count;

    let mut y = bounds.y;
    for (index, child) in children.iter().enumerate() {
        let extra = u16::from((index as u16) < remainder);
        let height = cell_height + extra;
        let child_rect = Rect {
            x: bounds.x,
            y,
            width: bounds.width,
            height,
        };
        layout_widget(child, child_rect, frame, seen_ids)?;
        y = y.saturating_add(height).saturating_add(spacing);
    }

    Ok(())
}

fn center(rect: Rect) -> (i32, i32) {
    (
        i32::from(rect.x) + i32::from(rect.width / 2),
        i32::from(rect.y) + i32::from(rect.height / 2),
    )
}

pub(crate) fn text_height(font: Font) -> u16 {
    match font {
        Font::Font8 => 8,
        Font::Font12 => 12,
        Font::Font16 => 16,
        Font::Font20 => 20,
        Font::Font24 => 24,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Button, ButtonStyle, Column, Row, Theme, Widget};

    fn bounds() -> Rect {
        Rect {
            x: 0,
            y: 0,
            width: 128,
            height: 128,
        }
    }

    fn style() -> ButtonStyle {
        ButtonStyle::themed(Theme::default())
    }

    #[test]
    fn row_layout_splits_width() {
        let widget = Widget::Row(Row::new(vec![
            Widget::Button(Button::new(1, "A", style())),
            Widget::Button(Button::new(2, "B", style())),
        ]));

        let frame = build_layout(&widget, bounds()).expect("row layout should work");
        assert_eq!(frame.buttons.len(), 2);
        assert_eq!(
            frame.buttons[0].rect.width + frame.buttons[1].rect.width,
            128
        );
    }

    #[test]
    fn column_layout_splits_height() {
        let widget = Widget::Column(Column::new(vec![
            Widget::Button(Button::new(1, "A", style())),
            Widget::Button(Button::new(2, "B", style())),
        ]));

        let frame = build_layout(&widget, bounds()).expect("column layout should work");
        assert_eq!(frame.buttons.len(), 2);
        assert_eq!(
            frame.buttons[0].rect.height + frame.buttons[1].rect.height,
            128
        );
    }

    #[test]
    fn detects_duplicate_widget_id() {
        let widget = Widget::Column(Column::new(vec![
            Widget::Button(Button::new(1, "A", style())),
            Widget::Button(Button::new(1, "B", style())),
        ]));

        let result = build_layout(&widget, bounds());
        assert!(matches!(result, Err(UiError::DuplicateWidgetId(1))));
    }

    #[test]
    fn initial_focus_picks_first_button() {
        let widget = Widget::Column(Column::new(vec![
            Widget::Button(Button::new(10, "A", style())),
            Widget::Button(Button::new(20, "B", style())),
        ]));

        let frame = build_layout(&widget, bounds()).expect("column layout should work");
        assert_eq!(initial_focus(&frame), Some(10));
    }

    #[test]
    fn geo_focus_moves_in_all_directions() {
        let widget = Widget::Column(Column::new(vec![
            Widget::Row(Row::new(vec![
                Widget::Button(Button::new(1, "TL", style())),
                Widget::Button(Button::new(2, "TR", style())),
            ])),
            Widget::Row(Row::new(vec![
                Widget::Button(Button::new(3, "BL", style())),
                Widget::Button(Button::new(4, "BR", style())),
            ])),
        ]));

        let frame = build_layout(&widget, bounds()).expect("grid layout should work");

        assert_eq!(move_focus(&frame, Some(1), FocusDirection::Right), Some(2));
        assert_eq!(move_focus(&frame, Some(1), FocusDirection::Down), Some(3));
        assert_eq!(move_focus(&frame, Some(4), FocusDirection::Left), Some(3));
        assert_eq!(move_focus(&frame, Some(4), FocusDirection::Up), Some(2));
    }

    #[test]
    fn focus_keeps_current_when_no_candidate() {
        let widget = Widget::Button(Button::new(1, "Only", style()));
        let frame = build_layout(&widget, bounds()).expect("single button layout should work");

        assert_eq!(move_focus(&frame, Some(1), FocusDirection::Up), Some(1));
    }
}
