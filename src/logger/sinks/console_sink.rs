use super::super::sink::Sink;
use strfmt::{strfmt, strfmt_builder};

pub struct ConsoleSink {
    level_filter: log::LevelFilter,
    format: String,
    // datetime_format: chrono::format::StrftimeItems<'static>,
    color: bool,
}

impl ConsoleSink {
    pub fn new() -> Self {
        Self {
            level_filter: log::LevelFilter::Trace,
            format: String::from("[{datetime}] [{level:5}] {message}"),
            // datetime_format: chrono::format::StrftimeItems::new("%H:%M:%S.%3f"),
            color: true,
        }
    }

    pub fn with_level_filter(mut self, level_filter: log::LevelFilter) -> Self {
        self.level_filter = level_filter;
        self
    }

    pub fn with_format(mut self, format: impl Into<String>) -> Self {
        self.format = format.into();
        self
    }

    pub fn with_datetime_format(mut self, datetime_format: &'static str) -> Self {
        // self.datetime_format = chrono::format::StrftimeItems::new(&datetime_format);
        self
    }

    fn color_level(level: log::Level) -> &'static str {
        match level {
            log::Level::Error => "\x1b[31m",
            log::Level::Warn => "\x1b[33m",
            log::Level::Info => "\x1b[34m",
            log::Level::Debug => "\x1b[35m",
            log::Level::Trace => "\x1b[32m",
        }
    }
}

impl Sink for ConsoleSink {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn log(&self, record: &log::Record) {
        // let datetime = chrono::Local::now().format_with_items(self.datetime_format.clone());

        let level = match self.color {
            true => format!(
                "{}{}\x1b[0m",
                Self::color_level(record.level()),
                record.level()
            ),
            false => record.level().to_string(),
        };

        // let datetime = format!("\x1b[2m{}\x1b[0m", datetime);

        let message = strfmt!(
            &self.format,
            // datetime => datetime,
            level => level,
            message => record.args().to_string(),
            file => record.file().unwrap_or_default().to_string(),
            line => record.line().unwrap_or_default(),
            module_path => record.module_path().unwrap_or_default().to_string(),
            target => record.target().to_string()
        );

        match message {
            Ok(message) => println!("{}", message),
            Err(err) => eprintln!("Failed to format message: {}", err),
        }
    }

    fn flush(&self) {}
}
