#[derive(Default)]
pub enum CurrentScreen {
    #[default]
    Left,
    Middle,
    Right,
}

#[derive(Default)]
pub struct App {
    pub current_screen: CurrentScreen, // Current screen that is active
}

#[cfg(test)]
mod test {
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    use super::*;

    #[test]
    fn test_render_app() {
        let app = App::default();
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| {
                crate::ui::setup_ui(frame, &app);
            })
            .unwrap();
        assert_snapshot!(terminal.backend())
    }
}
