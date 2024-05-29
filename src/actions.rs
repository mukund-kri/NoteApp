use std::path::PathBuf;
use std::{error, fmt};

use regex::Regex;

#[derive(Debug, Clone)]
pub struct InvalidDateError;

impl fmt::Display for InvalidDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid date format")
    }
}

impl error::Error for InvalidDateError {}

#[derive(Debug, Clone)]
pub struct NoteDate {
    pub year: i16,
    pub month: Option<i8>,
    pub day: Option<i8>,
}

impl NoteDate {
    pub fn year_only(year: i16) -> NoteDate {
        NoteDate {
            year,
            month: None,
            day: None,
        }
    }

    pub fn year_month(year: i16, month: i8) -> NoteDate {
        NoteDate {
            year,
            month: Some(month),
            day: None,
        }
    }

    pub fn full_date(year: i16, month: i8, day: i8) -> NoteDate {
        NoteDate {
            year,
            month: Some(month),
            day: Some(day),
        }
    }

    pub fn to_path(&self) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(self.year.to_string());
        if let Some(month) = self.month {
            path.push(format!("{:02}", month));
            if let Some(day) = self.day {
                path.push(format!("{:02}", day));
            } else {
                path.push("unclassified");
            }
        } else {
            path.push("unclassified");
        }
        path
    }

    pub fn validate(date: &str) -> Result<NoteDate, InvalidDateError> {
        // full date regex
        let full_date_re =
            Regex::new(r"^(?P<year>\d{4}).(?P<month>\d{2}).(?P<day>\d{2})$").unwrap();

        let caps = full_date_re.captures(date);
        if let Some(caps) = caps {
            let year = caps["year"].parse().unwrap();
            let month = caps["month"].parse().unwrap();
            let day = caps["day"].parse().unwrap();
            return Ok(NoteDate::full_date(year, month, day));
        }

        // year and month regex
        let year_month_re = Regex::new(r"(?P<year>\d{4}).(?P<month>\d{2})$").unwrap();

        let caps = year_month_re.captures(date);
        if let Some(caps) = caps {
            let year = caps["year"].parse().unwrap();
            let month = caps["month"].parse().unwrap();
            return Ok(NoteDate::year_month(year, month));
        }

        // year only regex
        let year_re = Regex::new(r"^(?P<year>\d{4})$").unwrap();

        let caps = year_re.captures(date);

        if let Some(caps) = caps {
            let year = caps["year"].parse().unwrap();
            return Ok(NoteDate::year_only(year));
        }

        Err(InvalidDateError)
    }
}

mod test {
    use super::*;

    #[test]
    fn test_type() {
        let nd1 = NoteDate {
            year: 2021,
            month: Some(1),
            day: Some(1),
        };

        assert!(nd1.year == 2021);
        assert!(nd1.month == Some(1));
        assert!(nd1.day == Some(1));
    }

    #[test]
    fn test_constructors() {
        let year_only = NoteDate::year_only(2021);
        assert!(year_only.year == 2021);

        let year_month = NoteDate::year_month(2021, 1);
        assert!(year_month.year == 2021);
        assert!(year_month.month == Some(1));

        let full_date = NoteDate::full_date(2021, 1, 1);
        assert!(full_date.year == 2021);
        assert!(full_date.month == Some(1));
        assert!(full_date.day == Some(1));
    }

    #[test]
    fn test_to_path() {
        let year_only = NoteDate::year_only(2021);
        assert!(year_only.to_path() == PathBuf::from("2021"));

        let year_month = NoteDate::year_month(2021, 1);
        assert!(year_month.to_path() == PathBuf::from("2021/01"));

        let full_date = NoteDate::full_date(2021, 1, 1);
        assert!(full_date.to_path() == PathBuf::from("2021/01/01"));
    }

    #[test]
    fn test_invalid_date_error() {
        let date = NoteDate::validate("2000").unwrap();
        assert!(date.year == 2000);
        assert!(date.month == None);
        assert!(date.day == None);

        let date = NoteDate::validate("2000.01").unwrap();
        assert!(date.year == 2000);
        assert!(date.month == Some(1));
        assert!(date.day == None);

        let date = NoteDate::validate("2000.01.01").unwrap();
        assert!(date.year == 2000);
        assert!(date.month == Some(1));
        assert!(date.day == Some(1));

        // Test the error cases where InvalidDateError is returned
        let date = NoteDate::validate("2000.01.01.01");
        assert!(date.is_err());
        assert!(date.unwrap_err().to_string() == "Invalid date format");

        let date = NoteDate::validate("20");
        assert!(date.is_err());
        assert!(date.unwrap_err().to_string() == "Invalid date format");
    }
}
