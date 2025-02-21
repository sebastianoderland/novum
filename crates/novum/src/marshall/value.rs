use std::collections::HashMap;

// use time::OffsetDateTime;
// use time::PrimitiveDateTime;

// pub enum DateTime {
//     Offset(OffsetDateTime),
//     Primitive(PrimitiveDateTime),
// }

pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    // Either OffsetDateTime or PrimitiveDateTime
    // DateTime(OffsetDateTime),
}
