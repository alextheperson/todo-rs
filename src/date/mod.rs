use chrono::Datelike;
use chrono::NaiveDate;
use chrono::offset::Local;

use crate::error::CodeComponent;
use crate::error::Error;
use crate::match_error;
use crate::match_option;
use crate::match_result;

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
    pub fn from_date(date: chrono::NaiveDate) -> Result<Date, Error> {
        Ok(Date {
            day: match_result!(
                usize::try_from(date.day()),
                CodeComponent::Date,
                format!("Could not parse the day '{}'", date.day())
            ),
            month: match_result!(
                usize::try_from(date.month()),
                CodeComponent::Date,
                format!("Could not parse the month '{}'", date.month())
            ),
            year: match_result!(
                u16::try_from(date.year()),
                CodeComponent::Date,
                format!("Could not parse the year '{}'", date.year())
            ),
        })
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

    pub fn today() -> Result<Date, Error> {
        Date::from_date(Local::now().date_naive())
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

    pub fn as_chrono(&self) -> Result<NaiveDate, Error> {
        Ok(match_option!(
            NaiveDate::from_ymd_opt(
                self.year.into(),
                match_result!(
                    self.month.try_into(),
                    CodeComponent::Date,
                    format!("Could not coerce {} to month.", self.month)
                ),
                match_result!(
                    self.day.try_into(),
                    CodeComponent::Date,
                    format!("Could not coerce {} to day.", self.day)
                ),
            ),
            CodeComponent::Date,
            format!("Could not convert to 'chrono::NaiveDate'.")
        ))
    }

    pub fn distance(&self, relative: Date) -> Result<i64, Error> {
        let self_date = match_error!(
            self.as_chrono(),
            CodeComponent::Date,
            format!("Could not convert date.")
        );
        let other_date = match_error!(
            relative.as_chrono(),
            CodeComponent::Date,
            format!("Could not convert date.")
        );

        Ok(self_date.signed_duration_since(other_date).num_days())
    }
}
