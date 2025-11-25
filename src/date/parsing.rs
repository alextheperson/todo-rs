use crate::date::Date;
use crate::error::{CodeComponent, CodeComponent::DateParser, Error};
use crate::{match_error, match_result, propagate};

impl Date {
    pub fn from(date: &str) -> Result<Date, Error> {
        if let Ok(relative) = Date::parse_relative_date(date) {
            return Ok(relative);
        }

        let slash_separated = date.split("/").collect::<Vec<&str>>();
        let dash_separated = date.split("-").collect::<Vec<&str>>();
        let space_separated = date.split(" ").collect::<Vec<&str>>();

        let items: Vec<&str>;

        // Prefer the separator that gives three items, and prefer slashes first. If nothing gives
        // three, check for two
        if slash_separated.len() == 3 {
            items = slash_separated;
        } else if dash_separated.len() == 3 {
            items = dash_separated;
        } else if space_separated.len() == 3 {
            items = space_separated;
        } else if slash_separated.len() == 2 {
            items = slash_separated;
        } else if dash_separated.len() == 2 {
            items = dash_separated;
        } else if space_separated.len() == 2 {
            items = space_separated;
        } else {
            println!("[DATE]: Had trouble parsing the date '{}'", date);
            return Err(propagate!(
                DateParser,
                format!(
                    "The date '{date}' could not be split up into 2 or 3 pieces. Possible delimiters are '/', ' ', and '-'.",
                )
            ));
        }

        if items.len() == 3 {
            return Ok(match_error!(
                Date::parse_three_values(items[0], items[1], items[2]),
                DateParser,
                format!("Could not parse the date '{date}' with 3 words.")
            ));
        } else if items.len() == 2 {
            return Ok(match_error!(
                Date::parse_two_values(items[0], items[1]),
                DateParser,
                format!("Could not parse the date '{date}' with 2 words.")
            ));
        }

        return Err(propagate!(
            DateParser,
            format!(
                "There were 2 or 3 words, but then there were neither 2 or 3 words on the date {date}. This is really bad."
            )
        ));
    }
    fn parse_three_values(w1: &str, w2: &str, w3: &str) -> Result<Date, Error> {
        // Prefer DMY then MDY, then YMD
        if let Ok(date) = Date::parse_dmy(w1, w2, w3) {
            return Ok(date);
        };

        if let Ok(date) = Date::parse_dmy(w2, w1, w3) {
            return Ok(date);
        };

        if let Ok(date) = Date::parse_dmy(w3, w2, w1) {
            return Ok(date);
        };

        Err(propagate!(
            DateParser,
            format!(
                "Could not parse the three-word date as DMY, MDY, or YMD. Words are: '{w1}', '{w2}', '{w3}'."
            )
        ))
    }

    fn parse_two_values(w1: &str, w2: &str) -> Result<Date, Error> {
        let today = match_error!(
            Date::today(),
            DateParser,
            format!("Couldn't get today's date.")
        );
        let default_day = "1";
        let default_month = today.month;
        let default_month_string = default_month.to_string();
        let default_year = today.year;
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
            if Date::parse_day(w2, default_year).unwrap_or(1) > today.day {
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
            if Date::parse_day(w1, default_year).unwrap_or(1) > today.day {
                default_year.to_string()
            } else {
                default_next_year.to_string()
            }
        } else {
            default_next_year.to_string()
        });

        if let Ok(date) = Date::parse_dmy(w1, w2, v2_year) {
            return Ok(date);
        };

        if let Ok(date) = Date::parse_dmy(w1, &default_month_string, w2) {
            return Ok(date);
        };

        if let Ok(date) = Date::parse_dmy(w2, w1, v1_year) {
            return Ok(date);
        };

        if let Ok(date) = Date::parse_dmy(&default_day, w1, w2) {
            return Ok(date);
        };

        if let Ok(date) = Date::parse_dmy(&default_day, w2, w1) {
            return Ok(date);
        };

        if let Ok(date) = Date::parse_dmy(w2, &default_month_string, w1) {
            return Ok(date);
        };

        Err(propagate!(
            DateParser,
            format!("Could not parse the two-word date. Words: '{w1}', '{w2}'.")
        ))
    }

    fn parse_dmy(d: &str, m: &str, y: &str) -> Result<Date, Error> {
        let year = match_error!(
            Date::parse_year(y),
            DateParser,
            format!("Could not parse '{y}' as a year.")
        );
        let month = match_error!(
            Date::parse_month(m),
            DateParser,
            format!("Could not parse '{m}' as a month.")
        );
        let day = match_error!(
            Date::parse_day(d, year),
            DateParser,
            format!("Could not parse '{d}' as a day.")
        );

        if !match_result!(
            Date::validate_month_length(month, year, &day),
            DateParser,
            format!("Could not validate the day number vs the length of the month.")
        ) {
            return Err(propagate!(
                DateParser,
                format!("Day {day} was too big for month {month} in year {year}.")
            ));
        }

        Ok(Date {
            day: day,
            month: month,
            year: year,
        })
    }

    fn parse_day(day: &str, year: u16) -> Result<usize, Error> {
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

        let value;
        if let Ok(day) = number {
            value = day
        } else {
            if let Ok(Ok(day)) = two_numbers {
                value = day
            } else {
                if let Ok(Ok(day)) = one_number {
                    value = day
                } else {
                    return Err(propagate!(
                        DateParser,
                        format!("Could not parse the day '{day}' in the year '{year}'")
                    ));
                }
            }
        }

        if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
            if value > 366 {
                return Err(propagate!(
                    DateParser,
                    format!("Day was out of range ({value} > 366).")
                ));
            }
        } else {
            if value > 365 {
                return Err(propagate!(
                    DateParser,
                    format!("Day was out of range ({value} > 365).")
                ));
            }
        }

        Ok(value)
    }

    pub fn parse_month(month: &str) -> Result<usize, Error> {
        if month == "" {
            return Err(propagate!(
                DateParser,
                format!("Cannot parse empty string as month")
            ));
        }

        let month_names = [
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

        if let Ok(parsed) = month.parse::<usize>() {
            let number = parsed;
            if number > 1 && number <= 12 {
                return Ok(number);
            } else {
                return Err(propagate!(
                    DateParser,
                    format!("Month {number} was out of range")
                ));
            }
        } else {
            for (i, possible_month) in month_names.into_iter().enumerate() {
                if possible_month.starts_with(&month.to_ascii_lowercase()) {
                    return Ok(i + 1);
                }
            }
        }

        Err(propagate!(
            DateParser,
            format!("Could not parse the month '{month}'")
        ))
    }

    fn parse_year(year: &str) -> Result<u16, Error> {
        let parsed = match_result!(
            year.parse::<u16>(),
            DateParser,
            format!("Couldn't parse year number ('{year}')")
        );

        // If you write, say 25, it will convert it to 2025. This will need to be updated in
        // 975 years
        if parsed < 1000 {
            return Ok(parsed + 2000);
        } else {
            return Ok(parsed);
        }
    }
}
