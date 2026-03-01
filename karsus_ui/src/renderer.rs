use crate::config::Theme;
use crate::error::UiResult;
use crate::layout::{text_height, ButtonLayout, LayoutFrame};
use crate::widget::WidgetId;

pub fn render(
    lcd: &mut karsus_ui_backend::LcdHat,
    frame: &LayoutFrame,
    theme: Theme,
    focused: Option<WidgetId>,
) -> UiResult<()> {
    lcd.clear(theme.background)?;

    for text in &frame.texts {
        let foreground = text.color.unwrap_or(theme.primary);
        lcd.draw_text(
            text.rect.x,
            text.rect.y,
            &text.text,
            text.font,
            foreground,
            theme.background,
        )?;
    }

    for button in &frame.buttons {
        draw_button(lcd, button, focused == Some(button.id))?;
    }

    lcd.present()?;
    Ok(())
}

fn draw_button(
    lcd: &mut karsus_ui_backend::LcdHat,
    button: &ButtonLayout,
    is_focused: bool,
) -> UiResult<()> {
    let palette = if is_focused {
        button.style.focused
    } else {
        button.style.normal
    };

    let x0 = button.rect.x;
    let y0 = button.rect.y;
    let x1 = x0.saturating_add(button.rect.width.saturating_sub(1));
    let y1 = y0.saturating_add(button.rect.height.saturating_sub(1));

    lcd.draw_rect(x0, y0, x1, y1, palette.background, 1, true)?;
    lcd.draw_rect(
        x0,
        y0,
        x1,
        y1,
        palette.border,
        button.style.border_width,
        false,
    )?;

    let text_y = y0
        .saturating_add(button.style.padding)
        .min(y1.saturating_sub(text_height(karsus_ui_backend::Font::Font12)));
    let text_x = x0.saturating_add(button.style.padding).min(x1);

    lcd.draw_text(
        text_x,
        text_y,
        &button.label,
        karsus_ui_backend::Font::Font12,
        palette.foreground,
        palette.background,
    )?;

    Ok(())
}
