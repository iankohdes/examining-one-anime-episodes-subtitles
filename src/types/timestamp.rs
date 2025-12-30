use std::str::FromStr;

const PERMITTED_TIMESTAMP_CHARS: &str = "0123456789:,";
const U8_MAX_255: usize = u8::MAX as usize;
const U16_MAX_65535: usize = u16::MAX as usize;

///Rich timestamp representation.
///Represents `hh:mm:ss,mmm`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
    pub milliseconds: u16,
}

#[derive(Debug)]
pub enum TimestampError {
    EmptyString,
    MalformedTimestamp(String),
}

impl TimestampError {
    fn malformed(msg: &str, original_input: &str) -> Self {
        TimestampError::MalformedTimestamp(
            format!("{} (input string: {})", msg, original_input)
        )
    }
}

impl FromStr for Timestamp {
    type Err = TimestampError;

    /// Allowed characters in a timestamp are: `01234567890:,`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn count_char_occurrences(xs: &str, x: char) -> usize {
            xs.chars().filter(|&char| char == x).count()
        }

        if s.is_empty() {
            return Err(TimestampError::EmptyString)
        } else if count_char_occurrences(s, ',') != 1 {
            return Err(TimestampError::malformed("Timestamp string can’t have more than one comma as a millisecond separator", s))
        } else if s.contains("\n") {
            return Err(TimestampError::malformed("Timestamp string cannot contain newlines", s))
        } else if s.chars().all(|char| PERMITTED_TIMESTAMP_CHARS.contains(char)) == false {
            return Err(TimestampError::malformed("Illegal characters detected; allowed characters are 0123456789:,", s))
        }

        // First element of vector: HH:MM:SS
        // Second element of vector: MS
        let split_on_comma: Vec<&str> = s.split(",").collect();

        let raw_hh_mm_ss = split_on_comma[0];
        let split_hh_mm_ss: Vec<&str> = raw_hh_mm_ss.split(":").collect();
        if split_hh_mm_ss.len() != 3 {
            return Err(TimestampError::malformed("Timestamp must have properly defined HH:MM:SS component", s))
        }

        let raw_hh = split_hh_mm_ss[0];
        let raw_mm = split_hh_mm_ss[1];
        let raw_ss = split_hh_mm_ss[2];
        if raw_hh.is_empty() {
            return Err(TimestampError::malformed("Empty hours value in HH:MM:SS", s))
        } else if raw_mm.is_empty() {
            return Err(TimestampError::malformed("Empty minutes value in HH:MM:SS", s))
        } else if raw_ss.is_empty() {
            return Err(TimestampError::malformed("Empty seconds value in HH:MM:SS", s))
        }

        let raw_ms = split_on_comma[1];
        if raw_ms.is_empty() {
            return Err(TimestampError::malformed("Empty milliseconds value", s))
        } else if count_char_occurrences(raw_ms, ':') > 0 {
            return Err(TimestampError::malformed("Milliseconds component contains illegal colon character", s))
        }

        let hours = raw_hh.parse::<usize>().unwrap();
        if hours > U8_MAX_255 { // Should I increase the allocation to u16?
            return Err(TimestampError::malformed("Hours value exceeds maximum unsigned 8-bit value of 255", s))
        }
        let minutes = raw_mm.parse::<usize>().unwrap();
        if minutes > U8_MAX_255 {
            return Err(TimestampError::malformed("Minutes value exceeds maximum unsigned 8-bit value of 255", s))
        } else if minutes > 59 {
            return Err(TimestampError::malformed("Minutes value exceeds 59", s))
        }
        let seconds = raw_ss.parse::<usize>().unwrap();
        if seconds > U8_MAX_255 {
            return Err(TimestampError::malformed("Seconds value exceeds maximum unsigned 8-bit value of 255", s))
        } else if seconds > 59 {
            return Err(TimestampError::malformed("Seconds value exceeds 59", s))
        }
        let milliseconds = raw_ms.parse::<usize>().unwrap();
        if milliseconds > U16_MAX_65535 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::malformed("Milliseconds value exceeds maximum unsigned 16-bit value of 65535", s))
        } else if milliseconds > 999 {
            return Err(TimestampError::malformed("Milliseconds value exceeds 999", s))
        }

        Ok(
            Timestamp{
                hours: hours as u8,
                minutes: minutes as u8,
                seconds: seconds as u8,
                milliseconds: milliseconds as u16
            }
        )
    }
}