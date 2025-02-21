use super::sink::Sink;

pub use log::Level;
pub use log::LevelFilter;

pub struct Logger {
    sinks: Vec<Box<dyn Sink>>,
    level_filter: log::LevelFilter,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            sinks: Vec::new(),
            level_filter: log::LevelFilter::Trace,
        }
    }

    pub fn with_level_filter(mut self, level_filter: log::LevelFilter) -> Self {
        self.level_filter = level_filter;
        self
    }

    pub fn with_sink(mut self, sink: Box<dyn Sink>) -> Self {
        self.sinks.push(sink);
        self
    }

    pub fn install(self) -> Result<(), log::SetLoggerError> {
        let max_level = self.level_filter;
        log::set_boxed_logger(Box::new(self))?;
        log::set_max_level(max_level);
        Ok(())
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level_filter
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            for sink in &self.sinks {
                if sink.enabled(record.metadata()) {
                    sink.log(record);
                }
            }
        }
    }

    fn flush(&self) {
        for sink in &self.sinks {
            sink.flush();
        }
    }
}
