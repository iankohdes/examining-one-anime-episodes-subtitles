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
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Timestamp string must have one, and only one, comma as a millisecond separator")))
        } else if s.contains("\n") {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Illegal newline character detected")))
        } else if s.chars().all(|char| PERMITTED_TIMESTAMP_CHARS.contains(char)) == false {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Illegal characters detected; allowed characters are 0123456789:,")))
        }

        // First element of vector: HH:MM:SS
        // Second element of vector: MS
        let split_on_comma: Vec<&str> = s.split(",").collect();

        let raw_hh_mm_ss = split_on_comma[0];
        let split_hh_mm_ss: Vec<&str> = raw_hh_mm_ss.split(":").collect();
        if split_hh_mm_ss.len() != 3 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Timestamp must have properly defined HH:MM:SS component")))
        }

        let raw_hh = split_hh_mm_ss[0];
        let raw_mm = split_hh_mm_ss[1];
        let raw_ss = split_hh_mm_ss[2];
        if raw_hh.is_empty() {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Empty hours value in HH:MM:SS")))
        } else if raw_mm.is_empty() {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Empty minutes value in HH:MM:SS")))
        } else if raw_ss.is_empty() {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Empty seconds value in HH:MM:SS")))
        }

        let raw_ms = split_on_comma[1];
        if raw_ms.is_empty() {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Empty milliseconds value")))
        } else if count_char_occurrences(raw_ms, ':') > 0 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Milliseconds component contains illegal colon character")))
        }

        let hours = raw_hh.parse::<usize>().unwrap();
        if hours > U8_MAX_255 { // Should I increase the allocation to u16? 255 hours may be exceeded in very rare cases
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Hours value exceeds maximum unsigned 8-bit value of 255")))
        }
        let minutes = raw_mm.parse::<usize>().unwrap();
        if minutes > U8_MAX_255 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Minutes value exceeds maximum unsigned 8-bit value of 255")))
        } else if minutes > 59 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Minutes value exceeds 59")))
        }
        let seconds = raw_ss.parse::<usize>().unwrap();
        if seconds > U8_MAX_255 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Seconds value exceeds maximum unsigned 8-bit value of 255")))
        } else if seconds > 59 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Seconds value exceeds 59")))
        }
        let milliseconds = raw_ms.parse::<usize>().unwrap();
        if milliseconds > U16_MAX_65535 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Milliseconds value exceeds maximum unsigned 16-bit value of 65535")))
        } else if milliseconds > 999 {
            println!("Regarding timestamp string: {}", s);
            return Err(TimestampError::MalformedTimestamp(String::from("Milliseconds value exceeds 999")))
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