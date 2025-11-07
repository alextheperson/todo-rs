use chrono::Datelike;
use chrono::offset::Local;

use crate::date::Date;

impl Date {
    pub fn from(date: &str) -> Result<Date, &str> {
        let relative = Date::parse_relative_date(date);
        if relative.is_ok() {
            return Ok(relative.unwrap());
        }

        let slash_separated = date.split("/");
        let dash_separated = date.split("-");
        let space_separated = date.split(" ");

        let items: Vec<&str>;

        // Prefer the separator that gives three items, and prefer slashes first. If nothing gives
        // three, check for two
        if slash_separated.clone().count() == 3 {
            items = slash_separated.collect();
        } else if dash_separated.clone().count() == 3 {
            items = dash_separated.collect();
        } else if space_separated.clone().count() == 3 {
            items = space_separated.collect();
        } else if slash_separated.clone().count() == 2 {
            items = slash_separated.collect();
        } else if dash_separated.clone().count() == 2 {
            items = dash_separated.collect();
        } else if space_separated.clone().count() == 2 {
            items = space_separated.collect();
        } else {
            println!("[DATE]: Had trouble parsing the date '{}'", date);
            return Err("Too many (or too few) separators.");
        }

        if items.len() == 3 {
            return Date::parse_three_values(items[0], items[1], items[2]);
        } else if items.len() == 2 {
            return Date::parse_two_values(items[0], items[1]);
        }

        return Err("Something went deeply wrong. Please report this.");
    }
    fn parse_three_values<'a>(w1: &'a str, w2: &'a str, w3: &'a str) -> Result<Date, &'a str> {
        // Prefer DMY then MDY, then YMD
        let mut date = Date::parse_dmy(w1, w2, w3);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        date = Date::parse_dmy(w2, w1, w3);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        date = Date::parse_dmy(w3, w2, w1);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        Err("Could not parse the date.")
    }

    fn parse_two_values<'a>(w1: &'a str, w2: &'a str) -> Result<Date, &'a str> {
        let default_day = "1";
        let default_month = usize::try_from(Local::now().date_naive().month()).unwrap();
        let default_month_string = default_month.to_string();
        let default_year = u16::try_from(Local::now().date_naive().year()).unwrap();
        let default_next_year = default_year + 1;

        /*
        Possibilities (in order) + inference
        dm - current year (if in future)
        dy - current month (if in future)
        md - current year (if in future)
        my - 1st of month
        ym - 1st of month
        yd - current month (if in future)

        it should generally go less specific: month -> day -> year, so that one doesn't occlude
        the others.
        */

        let v1_month = Date::parse_month(w1).unwrap_or(1);
        let v2_month = Date::parse_month(w2).unwrap_or(1);

        let v1_year = &(if v1_month > default_month {
            default_year.to_string()
        } else if v1_month == default_month {
            if Date::parse_day(w2, default_year).unwrap_or(1)
                > usize::try_from(Local::now().date_naive().day()).unwrap()
            {
                default_year.to_string()
            } else {
                default_next_year.to_string()
            }
        } else {
            default_next_year.to_string()
        });
        let v2_year = &(if v2_month > default_month {
            default_year.to_string()
        } else if v2_month == default_month {
            if Date::parse_day(w1, default_year).unwrap_or(1)
                > usize::try_from(Local::now().date_naive().day()).unwrap()
            {
                default_year.to_string()
            } else {
                default_next_year.to_string()
            }
        } else {
            default_next_year.to_string()
        });

        let mut date = Date::parse_dmy(w1, w2, v2_year);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        date = Date::parse_dmy(w1, &default_month_string, w2);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        date = Date::parse_dmy(w2, w1, v1_year);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        date = Date::parse_dmy(&default_day, w1, w2);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        date = Date::parse_dmy(&default_day, w2, w1);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        date = Date::parse_dmy(w2, &default_month_string, w1);
        if date.is_ok() {
            return Ok(date.unwrap());
        };

        Err("Could not parse the two-word date")
    }

    fn parse_dmy<'a>(d: &'a str, m: &'a str, y: &'a str) -> Result<Date, &'a str> {
        let year = Date::parse_year(y)?;
        let month = Date::parse_month(m)?;
        let day = Date::parse_day(d, year)?;

        if !Date::validate_month_length(month, year, &day).unwrap() {
            return Err("Day was too big for the month.");
        }

        Ok(Date {
            day: day,
            month: month,
            year: year,
        })
    }

    fn parse_day(day: &str, year: u16) -> Result<usize, &str> {
        let number = day.parse::<usize>();
        let one_number = if day.len() > 1 {
            Ok(day[0..1].parse::<usize>())
        } else {
            Err(())
        };
        let two_numbers = if day.len() > 2 {
            Ok(day[0..2].parse::<usize>())
        } else {
            Err(())
        };

        let value = if number.is_ok() {
            number.unwrap()
        } else if two_numbers.is_ok() && two_numbers.as_ref().unwrap().is_ok() {
            two_numbers.unwrap().unwrap()
        } else if one_number.is_ok() && one_number.as_ref().unwrap().is_ok() {
            one_number.unwrap().unwrap()
        } else {
            return Err("Could not parse the day");
        };

        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            if value > 366 {
                return Err("Day was out of range.");
            }
        } else {
            if value > 365 {
                return Err("Day was out of range.");
            }
        }

        Ok(value)
    }

    fn parse_month(month: &str) -> Result<usize, &str> {
        let short_months = [
            "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
        ];
        let long_months = [
            "january",
            "february",
            "march",
            "april",
            "may",
            "june",
            "july",
            "august",
            "september",
            "october",
            "november",
            "december",
        ];

        if month.parse::<usize>().is_ok() {
            let number = month.parse::<usize>().unwrap();
            if number > 1 && number <= 12 {
                return Ok(number);
            } else {
                return Err("Month was out of range");
            }
        } else {
            if short_months.contains(&&month.to_ascii_lowercase()[..]) {
                return Ok(short_months
                    .iter()
                    .position(|s| s == &month.to_ascii_lowercase())
                    .unwrap()
                    + 1);
            } else if long_months.contains(&&month.to_ascii_lowercase()[..]) {
                return Ok(long_months
                    .iter()
                    .position(|s| s == &month.to_ascii_lowercase())
                    .unwrap()
                    + 1);
            }
        }

        Err("Could not parse the month")
    }

    fn parse_year(year: &str) -> Result<u16, &str> {
        if year.parse::<u16>().is_ok() {
            let parsed = year.parse::<u16>().unwrap();
            // If you write, say 25, it will convert it to 2025. This will need to be updated in
            // 975 years
            if parsed < 1000 {
                return Ok(parsed + 2000);
            } else {
                return Ok(parsed);
            }
        }

        Err("Could not parse the year.")
    }
}
