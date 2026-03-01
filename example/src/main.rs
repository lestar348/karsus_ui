use karsus_ui::{
    App, AppConfig, Button, ButtonStyle, Column, Page, PageCommand, Theme, UiError, UiEvent, Widget,
};

const BTN_OPEN_DETAILS: u32 = 1;
const BTN_NOOP: u32 = 2;

const ACTION_OPEN_DETAILS: u32 = 100;
const ACTION_NOOP: u32 = 101;

fn main() -> Result<(), UiError> {
    let config = AppConfig {
        theme: Theme {
            background: karsus_ui::color::WHITE,
            primary: karsus_ui::color::BLACK,
            on_primary: karsus_ui::color::WHITE,
            secondary: karsus_ui::color::BLUE,
            on_secondary: karsus_ui::color::WHITE,
        },
        ..AppConfig::default()
    };

    let mut app = App::new(config)?;
    app.push_page(Box::new(HomePage::default()));
    app.run()
}

#[derive(Default)]
struct HomePage {
    focused: Option<u32>,
    k2_hits: u32,
    k3_hits: u32,
}

impl Page for HomePage {
    fn title(&self) -> &str {
        "Home"
    }

    fn view(&self) -> Widget {
        let style = ButtonStyle::themed(Theme::default());
        Widget::Column(
            Column::new(vec![
                Widget::text("Karsus UI Demo"),
                Widget::Button(
                    Button::new(BTN_OPEN_DETAILS, "Open details", style)
                        .on_press(ACTION_OPEN_DETAILS),
                ),
                Widget::Button(Button::new(BTN_NOOP, "No-op", style).on_press(ACTION_NOOP)),
                Widget::text(format!("K2: {} K3: {}", self.k2_hits, self.k3_hits)),
            ])
            .spacing(4),
        )
    }

    fn on_event(&mut self, event: UiEvent) -> PageCommand {
        match event {
            UiEvent::Press {
                action: Some(ACTION_OPEN_DETAILS),
                ..
            } => PageCommand::Push(Box::new(DetailsPage::default())),
            UiEvent::Press {
                action: Some(ACTION_NOOP),
                ..
            } => PageCommand::None,
            UiEvent::K2 => {
                self.k2_hits = self.k2_hits.saturating_add(1);
                PageCommand::None
            }
            UiEvent::K3 => {
                self.k3_hits = self.k3_hits.saturating_add(1);
                PageCommand::None
            }
            _ => PageCommand::None,
        }
    }

    fn on_focus_changed(&mut self, focused_id: Option<u32>) {
        self.focused = focused_id;
    }
}

#[derive(Default)]
struct DetailsPage {
    k2_hits: u32,
    k3_hits: u32,
}

impl Page for DetailsPage {
    fn title(&self) -> &str {
        "Details"
    }

    fn view(&self) -> Widget {
        Widget::Column(
            Column::new(vec![
                Widget::text("Details page"),
                Widget::text("Press K1 to go back"),
                Widget::text(format!("K2: {} K3: {}", self.k2_hits, self.k3_hits)),
            ])
            .spacing(2),
        )
    }

    fn on_event(&mut self, event: UiEvent) -> PageCommand {
        match event {
            UiEvent::K2 => {
                self.k2_hits = self.k2_hits.saturating_add(1);
                PageCommand::None
            }
            UiEvent::K3 => {
                self.k3_hits = self.k3_hits.saturating_add(1);
                PageCommand::None
            }
            _ => PageCommand::None,
        }
    }
}
