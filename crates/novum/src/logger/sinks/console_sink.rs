use super::super::sink::Sink;
use crate::time::format_description::parse_owned;
use crate::time::format_description::OwnedFormatItem;
use crate::time::formatting::Formattable;
use crate::time::macros::format_description;
use strfmt::{strfmt, strfmt_builder};

pub struct ConsoleSink {
    level_filter: log::LevelFilter,
    format: String,
    datetime_format: OwnedFormatItem,
    color: bool,
}

impl ConsoleSink {
    pub fn new() -> Self {
        Self {
            level_filter: log::LevelFilter::Trace,
            format: String::from("[{datetime}] [{level:5}] {message}"),
            datetime_format: parse_owned::<2>("[hour]:[minute]:[second].[subsecond digits:6]")
                .unwrap(),
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

    pub fn with_datetime_format(mut self, datetime_format: impl AsRef<str>) -> Self {
        self.datetime_format = parse_owned::<2>(datetime_format.as_ref()).unwrap();
        self
    }

    pub fn with_color(mut self, color: bool) -> Self {
        self.color = color;
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
        let level = match self.color {
            true => format!(
                "{}{}\x1b[0m",
                Self::color_level(record.level()),
                record.level()
            ),
            false => record.level().to_string(),
        };

        #[cfg(feature = "logger-local-time")]
        let datetime = match crate::time::OffsetDateTime::now_local() {
            Ok(datetime) => datetime,
            Err(err) => {
                eprintln!("Failed to get datetime: {}", err);
                return;
            }
        };
        #[cfg(not(feature = "logger-local-time"))]
        let datetime = time::OffsetDateTime::now_utc();

        let datetime = format!(
            "\x1b[2m{}\x1b[0m",
            datetime.format(&self.datetime_format).unwrap()
        );

        let message = strfmt!(
            &self.format,
            datetime => datetime,
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
