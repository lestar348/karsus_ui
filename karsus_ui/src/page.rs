use crate::{UiEvent, Widget, WidgetId};

pub trait Page {
    fn title(&self) -> &str {
        ""
    }

    fn view(&self) -> Widget;

    fn on_event(&mut self, _event: UiEvent) -> PageCommand {
        PageCommand::None
    }

    fn on_focus_changed(&mut self, _focused_id: Option<WidgetId>) {}
}

pub enum PageCommand {
    None,
    Push(Box<dyn Page>),
    Pop,
    Replace(Box<dyn Page>),
    Exit,
}
