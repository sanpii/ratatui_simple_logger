fn main() -> Result<(), Box<dyn std::error::Error>> {
    ratatui_simple_logger::init()?;
    let terminal = ratatui::init();

    App::default().run(terminal)
}

#[derive(Default)]
struct App {
}

impl App {
    fn run(
        mut self,
        mut terminal: ratatui::DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        use ratatui::crossterm::event::{self, Event, KeyCode};

        let tick_rate = std::time::Duration::from_millis(250);
        let mut last_tick = std::time::Instant::now();

        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    log::debug!("{key:?}");

                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = std::time::Instant::now();
            }
        }
    }

    fn draw(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        let log = ratatui_simple_logger::Widget::new();
        frame.render_widget(log, area);
    }
}

impl Drop for App {
    fn drop(&mut self) {
        ratatui::restore();
    }
}
