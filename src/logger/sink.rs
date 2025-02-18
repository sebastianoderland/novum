pub trait Sink: Send + Sync {
    fn enabled(&self, metadata: &log::Metadata) -> bool;
    fn log(&self, record: &log::Record);
    fn flush(&self);
}
