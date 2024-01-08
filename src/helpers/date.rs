use std::fmt;

use chrono::NaiveDate;

pub struct RocDateString(pub String);

impl fmt::Display for RocDateString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<RocDateString> for Option<NaiveDate> {
    fn from(roc_date_string: RocDateString) -> Self {
        let (year, month, day) = parse_roc_date(&roc_date_string);
        NaiveDate::from_ymd_opt(year + 1911, month, day)
    }
}

fn parse_roc_date(date_str: &RocDateString) -> (i32, u32, u32) {
    let parts: Vec<&str> = date_str.0.split('.').collect();
    let year = parts[0].parse::<i32>().unwrap();
    let month = parts[1].parse::<u32>().unwrap();
    let day = parts[2].parse::<u32>().unwrap();
    (year, month, day)
}
