static LOGGER: std::sync::LazyLock<Logger> = std::sync::LazyLock::new(Logger::new);

pub fn init() -> Result<(), log::SetLoggerError> {
    if cfg!(debug_assertions) {
        log::set_max_level(log::LevelFilter::Trace);
    } else {
        log::set_max_level(log::LevelFilter::Warn);
    }
    log::set_logger(&*LOGGER)
}

pub fn set_max_level(level: log::LevelFilter) {
    log::set_max_level(level);
}

#[derive(Default)]
pub struct Logger {
    messages: std::sync::RwLock<Vec<Message>>,
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn messages(&self) -> Vec<Message> {
        self.messages.read().unwrap().clone()
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.target() == env!("CARGO_PKG_NAME")
    }

    fn log(&self, record: &log::Record) {
        self.messages.write().unwrap().push(record.into());
    }

    fn flush(&self) {}
}

#[derive(Clone)]
pub struct Message {
    level: log::Level,
    message: String,
}

impl<'a> From<&'a Message> for ratatui::widgets::ListItem<'a> {
    fn from(message: &'a Message) -> Self {
        use ratatui::style::{Color, Style};

        let s = match message.level {
            log::Level::Error => Style::default().fg(Color::Red),
            log::Level::Warn => Style::default().fg(Color::Yellow),
            log::Level::Info => Style::default().fg(Color::Green),
            log::Level::Debug => Style::default().fg(Color::Blue),
            log::Level::Trace => Style::default().fg(Color::Gray),
        };
        let span = ratatui::text::Line::from(vec![
            ratatui::text::Span::styled(format!("{:<9}", message.level), s),
            ratatui::text::Span::raw(" "),
            ratatui::text::Span::raw(message.message.as_str()),
        ]);
        ratatui::widgets::ListItem::new(span)
    }
}

impl<'a> From<&'a log::Record<'a>> for Message {
    fn from(record: &'a log::Record) -> Self {
        Self {
            level: record.level(),
            message: record.args().to_string(),
        }
    }
}

#[derive(Default)]
pub struct Widget;

impl Widget {
    pub fn new() -> Self {
        Self
    }
}

impl ratatui::widgets::Widget for Widget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let messages = LOGGER.messages();
        let block = ratatui::widgets::Block::bordered()
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("Logs");

        let list = ratatui::widgets::List::new(messages.iter()).block(block);
        ratatui::widgets::Widget::render(list, area, buf);
    }
}
