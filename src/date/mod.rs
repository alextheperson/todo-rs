use chrono::Datelike;
use chrono::NaiveDate;
use chrono::offset::Local;

pub mod parsing;
pub mod relative;

/// Represents a date that a list or item might be due.
/// It uses 1-based indexing for the days and months.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Date {
    day: usize,
    month: usize,
    year: u16,
}

impl Date {
    pub fn from_date(date: chrono::NaiveDate) -> Date {
        Date {
            day: usize::try_from(date.day()).unwrap(),
            month: usize::try_from(date.month()).unwrap(),
            year: u16::try_from(date.year()).unwrap(),
        }
    }

    fn validate_month_length(month: usize, year: u16, length: &usize) -> Result<bool, &str> {
        // IMPORTANT: remember February can't be checked like this
        let month_lengths = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if month > 12 {
            return Err("The month value is out of bounds");
        }

        // If it isn't February
        if month != 1 {
            return Ok(*length < month_lengths[month - 1]);
        } else {
            // leap year every 4 years, but skip every 100, and unskip every 400
            if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                return Ok(*length < 29);
            } else {
                return Ok(*length < 28);
            }
        }
    }

    pub fn today() -> Date {
        Date {
            day: usize::try_from(Local::now().date_naive().day()).unwrap(),
            month: usize::try_from(Local::now().date_naive().month()).unwrap(),
            year: u16::try_from(Local::now().date_naive().year()).unwrap(),
        }
    }

    pub fn month_to_short(month: &usize) -> &str {
        let short_months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];

        short_months[*month - 1]
    }

    pub fn display(&self) -> String {
        return format!(
            "{day}-{month}-{year}",
            day = self.day,
            month = Date::month_to_short(&self.month),
            year = self.year
        );
    }

    pub fn as_chrono(&self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(
            self.year.into(),
            self.month.try_into().unwrap(),
            self.day.try_into().unwrap(),
        )
    }

    pub fn distance(&self, relative: Date) -> i64 {
        let self_date = self.as_chrono().unwrap();
        let other_date = relative.as_chrono().unwrap();

        self_date.signed_duration_since(other_date).num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let goal_1 = Date {
            day: 10,
            month: 12,
            year: 2025,
        };

        let mut date = Date::from("10/12/2025").unwrap();
        assert_eq!(date, goal_1);

        date = Date::from("10-12-2025").unwrap();
        assert_eq!(date, goal_1);

        date = Date::from("2025-12-10").unwrap();
        assert_eq!(date, goal_1);

        date = Date::from("10/dec/2025").unwrap();
        assert_eq!(date, goal_1);

        date = Date::from("10 December 2025").unwrap();
        assert_eq!(date, goal_1);

        let goal_2 = Date {
            day: 1,
            month: 12,
            year: 2025,
        };

        date = Date::from("December 2025").unwrap();
        assert_eq!(date, goal_2);

        let goal_3 = Date {
            day: 5,
            month: 10,
            year: Date::today().year + 1,
        };

        date = Date::from("5 Oct").unwrap();
        assert_eq!(date, goal_3);

        let goal_4 = Date {
            day: Date::today().day + 1,
            month: Date::today().month,
            year: Date::today().year,
        };

        date = Date::from("tomorrow").unwrap();
        assert_eq!(date, goal_4);
    }
}
