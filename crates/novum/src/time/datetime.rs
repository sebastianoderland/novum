use extern_time::OffsetDateTime;
use extern_time::PrimitiveDateTime;
use extern_time::UtcOffset;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateTime {
    Offset(OffsetDateTime),
    Primitive(PrimitiveDateTime),
}

impl DateTime {
    pub fn now_utc() -> Self {
        DateTime::Offset(OffsetDateTime::now_utc())
    }

    #[cfg(feature = "time-local")]
    pub fn now_local() -> Self {
        DateTime::Offset(OffsetDateTime::now_local().unwrap())
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DateTime::Offset(dt) => write!(f, "{}", dt),
            DateTime::Primitive(dt) => write!(f, "{}", dt),
        }
    }
}

impl From<DateTime> for OffsetDateTime {
    fn from(dt: DateTime) -> Self {
        match dt {
            DateTime::Offset(dt) => dt,
            DateTime::Primitive(dt) => dt.assume_offset(UtcOffset::UTC),
        }
    }
}

impl From<DateTime> for PrimitiveDateTime {
    fn from(dt: DateTime) -> Self {
        match dt {
            DateTime::Offset(dt) => PrimitiveDateTime::new(dt.date(), dt.time()),
            DateTime::Primitive(dt) => dt,
        }
    }
}
