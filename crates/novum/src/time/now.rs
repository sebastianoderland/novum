use extern_time::OffsetDateTime;

#[cfg(feature = "time-local")]
pub fn now_local() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap()
}

pub fn now_utc() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}
