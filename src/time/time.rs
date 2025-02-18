#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    pub hour: u8,        // 0-23 | 5 bits
    pub minute: u8,      // 0-59 | 6 bits
    pub second: u8,      // 0-59 | 6 bits
    pub nanosecond: u32, // 0-999_999_999 | 30 bits
}

impl Time {
    pub fn new(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Self {
        Self {
            hour,
            minute,
            second,
            nanosecond,
        }
    }
}

// Implement the Display trait for Time
//
impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:09}",
            self.hour, self.minute, self.second, self.nanosecond
        )
    }
}

impl TryFrom<&str> for Time {
    type Error = std::num::ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(':');

        let hour = match parts.next() {
            None => 0,
            Some(part) => part.parse()?,
        };
        let minute = match parts.next() {
            None => 0,
            Some(part) => part.parse()?,
        };
        let (second, nanosecond): (u8, u32) = match parts.next() {
            None => (0u8, 0u32),
            Some(part) => {
                let mut parts = part.split('.');
                let second = parts.next().unwrap_or("0").parse()?;
                let nanosecond = match parts.next() {
                    None => 0,
                    Some(part) => {
                        let mut part = part.to_string();
                        while part.len() < 9 {
                            part.push('0');
                        }
                        part.parse()?
                    }
                };
                (second, nanosecond)
            }
        };

        Ok(Self {
            hour,
            minute,
            second,
            nanosecond,
        })
    }
}
