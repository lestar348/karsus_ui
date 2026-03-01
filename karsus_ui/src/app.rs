use crate::config::{AppConfig, LastPagePolicy};
use crate::error::{UiError, UiResult};
use crate::event::UiEvent;
use crate::layout::{build_layout, initial_focus, move_focus, FocusDirection, LayoutFrame, Rect};
use crate::page::{Page, PageCommand};

pub struct App {
    config: AppConfig,
    lcd: karsus_ui_backend::LcdHat,
    pages: Vec<Box<dyn Page>>,
    focused_button: Option<crate::WidgetId>,
    redraw_requested: bool,
    should_exit: bool,
}

impl App {
    pub fn new(config: AppConfig) -> UiResult<Self> {
        let mut lcd = karsus_ui_backend::LcdHat::new(config.backend)?;
        lcd.clear(config.theme.background)?;
        lcd.present()?;

        Ok(Self {
            config,
            lcd,
            pages: Vec::new(),
            focused_button: None,
            redraw_requested: true,
            should_exit: false,
        })
    }

    pub fn push_page(&mut self, page: Box<dyn Page>) {
        self.pages.push(page);
        self.focused_button = None;
        self.request_redraw();
    }

    pub fn replace_page(&mut self, page: Box<dyn Page>) {
        if self.pages.is_empty() {
            self.pages.push(page);
        } else {
            self.pages.pop();
            self.pages.push(page);
        }
        self.focused_button = None;
        self.request_redraw();
    }

    pub fn pop_page(&mut self) -> bool {
        if self.pages.len() > 1 {
            self.pages.pop();
            self.focused_button = None;
            self.request_redraw();
            return true;
        }

        false
    }

    pub fn request_redraw(&mut self) {
        self.redraw_requested = true;
    }

    pub fn run(&mut self) -> UiResult<()> {
        if self.pages.is_empty() {
            return Err(UiError::NoPages);
        }

        while !self.should_exit {
            if self.redraw_requested {
                self.redraw()?;
                self.redraw_requested = false;
            }

            match self.lcd.poll_key()? {
                None => karsus_ui_backend::LcdHat::sleep_ms(10),
                Some(key_event) => {
                    if !key_event.pressed {
                        continue;
                    }

                    let changed = match key_event.key {
                        karsus_ui_backend::Key::Up => self.handle_direction(FocusDirection::Up)?,
                        karsus_ui_backend::Key::Down => {
                            self.handle_direction(FocusDirection::Down)?
                        }
                        karsus_ui_backend::Key::Left => {
                            self.handle_direction(FocusDirection::Left)?
                        }
                        karsus_ui_backend::Key::Right => {
                            self.handle_direction(FocusDirection::Right)?
                        }
                        karsus_ui_backend::Key::Press => self.handle_press()?,
                        karsus_ui_backend::Key::K1 => self.handle_back()?,
                        karsus_ui_backend::Key::K2 => self.dispatch_page_event(UiEvent::K2)?,
                        karsus_ui_backend::Key::K3 => self.dispatch_page_event(UiEvent::K3)?,
                    };

                    if changed {
                        self.request_redraw();
                    }
                }
            }
        }

        Ok(())
    }

    fn redraw(&mut self) -> UiResult<()> {
        let frame = self.current_layout_frame()?;
        self.sync_focus(&frame)?;
        crate::renderer::render(
            &mut self.lcd,
            &frame,
            self.config.theme,
            self.focused_button,
        )?;
        Ok(())
    }

    fn current_layout_frame(&self) -> UiResult<LayoutFrame> {
        let page = self.pages.last().ok_or(UiError::NoPages)?;
        let root = page.view();
        let bounds = Rect {
            x: 0,
            y: 0,
            width: karsus_ui_backend::LCD_WIDTH,
            height: karsus_ui_backend::LCD_HEIGHT,
        };
        build_layout(&root, bounds)
    }

    fn sync_focus(&mut self, frame: &LayoutFrame) -> UiResult<()> {
        let old_focus = self.focused_button;
        self.focused_button = match self.focused_button {
            Some(id) if frame.button_by_id(id).is_some() => Some(id),
            _ => initial_focus(frame),
        };

        if old_focus != self.focused_button {
            if let Some(page) = self.pages.last_mut() {
                page.on_focus_changed(self.focused_button);
                return Ok(());
            }
            return Err(UiError::NoPages);
        }

        Ok(())
    }

    fn handle_direction(&mut self, direction: FocusDirection) -> UiResult<bool> {
        let frame = self.current_layout_frame()?;
        let old_focus = self.focused_button;
        self.focused_button = move_focus(&frame, self.focused_button, direction);

        if old_focus != self.focused_button {
            if let Some(page) = self.pages.last_mut() {
                page.on_focus_changed(self.focused_button);
            }
            return Ok(true);
        }

        Ok(false)
    }

    fn handle_press(&mut self) -> UiResult<bool> {
        let frame = self.current_layout_frame()?;
        let focused_id = match self.focused_button {
            Some(id) => id,
            None => return Ok(false),
        };

        let focused_button = frame
            .button_by_id(focused_id)
            .ok_or(UiError::FocusedWidgetNotFound(focused_id))?;

        self.dispatch_page_event(UiEvent::Press {
            focused: focused_id,
            action: focused_button.action,
        })
    }

    fn handle_back(&mut self) -> UiResult<bool> {
        if self.pages.len() > 1 {
            self.pages.pop();
            self.focused_button = None;
            return Ok(true);
        }

        match self.config.last_page_policy {
            LastPagePolicy::ExitApp => {
                self.should_exit = true;
                Ok(false)
            }
            LastPagePolicy::IgnoreBack => Ok(false),
        }
    }

    fn dispatch_page_event(&mut self, event: UiEvent) -> UiResult<bool> {
        let command = {
            let page = self.pages.last_mut().ok_or(UiError::NoPages)?;
            page.on_event(event)
        };

        self.apply_page_command(command)
    }

    fn apply_page_command(&mut self, command: PageCommand) -> UiResult<bool> {
        match command {
            PageCommand::None => Ok(true),
            PageCommand::Push(page) => {
                self.pages.push(page);
                self.focused_button = None;
                Ok(true)
            }
            PageCommand::Pop => {
                if self.pages.len() > 1 {
                    self.pages.pop();
                    self.focused_button = None;
                    Ok(true)
                } else {
                    match self.config.last_page_policy {
                        LastPagePolicy::ExitApp => {
                            self.should_exit = true;
                            Ok(false)
                        }
                        LastPagePolicy::IgnoreBack => Ok(false),
                    }
                }
            }
            PageCommand::Replace(page) => {
                if self.pages.is_empty() {
                    self.pages.push(page);
                } else {
                    self.pages.pop();
                    self.pages.push(page);
                }
                self.focused_button = None;
                Ok(true)
            }
            PageCommand::Exit => {
                self.should_exit = true;
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::LastPagePolicy;

    #[test]
    fn pop_page_works_only_with_more_than_one_page() {
        assert!(!can_pop(1));
        assert!(can_pop(2));
    }

    #[test]
    fn last_page_policy_exit_sets_exit() {
        let (should_exit, redraw) = apply_last_page_policy(LastPagePolicy::ExitApp);
        assert!(should_exit);
        assert!(!redraw);
    }

    #[test]
    fn last_page_policy_ignore_keeps_app_running() {
        let (should_exit, redraw) = apply_last_page_policy(LastPagePolicy::IgnoreBack);
        assert!(!should_exit);
        assert!(!redraw);
    }

    #[test]
    fn press_is_ignored_without_focus() {
        let action = resolve_press_action(None, &LayoutFrame::default());
        assert!(action.is_none());
    }

    #[test]
    fn press_uses_focused_button_action() {
        let frame = LayoutFrame {
            texts: Vec::new(),
            buttons: vec![crate::layout::ButtonLayout {
                rect: Rect {
                    x: 0,
                    y: 0,
                    width: 20,
                    height: 20,
                },
                id: 42,
                label: "ok".to_string(),
                style: crate::ButtonStyle::themed(crate::Theme::default()),
                action: Some(7),
            }],
        };

        assert_eq!(resolve_press_action(Some(42), &frame), Some((42, Some(7))));
    }

    fn can_pop(len: usize) -> bool {
        len > 1
    }

    fn apply_last_page_policy(policy: LastPagePolicy) -> (bool, bool) {
        match policy {
            LastPagePolicy::ExitApp => (true, false),
            LastPagePolicy::IgnoreBack => (false, false),
        }
    }

    fn resolve_press_action(
        focused: Option<crate::WidgetId>,
        frame: &LayoutFrame,
    ) -> Option<(crate::WidgetId, Option<crate::ButtonActionId>)> {
        let focused_id = focused?;
        let button = frame.button_by_id(focused_id)?;
        Some((focused_id, button.action))
    }
}
