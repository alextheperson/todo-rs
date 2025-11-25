use chrono::Datelike;
use chrono::Days;
use chrono::Months;
use chrono::offset::Local;

use crate::date::Date;
use crate::error::CodeComponent;
use crate::error::CodeComponent::DateParser;
use crate::error::Error;
use crate::match_error;
use crate::match_option;
use crate::match_result;
use crate::propagate;

impl Date {
    pub fn parse_relative_date(input: &str) -> Result<Date, Error> {
        /*
        Templates:

        |---------------|--------------------|
        | content       | pattern            |
        |---------------|--------------------|
        | tomorrow      | /tomorrow/         |
        | in {x} days   | /in [0-9]+ days/   |
        | next week     | /next week/        |
        | in 1 week     | /in 1 week/        |
        | in {x} weeks  | /in [0-9]+ weeks/  |
        | next month    | /next month/       |
        | in 1 month    | /in 1 month/       |
        | in {x} months | /in [0-9]+ months/ |
        | next year     | /next year/        |
        | in {x} years  | /in [0-9]+ years/  |
        |---------------|--------------------|

        |--------|---------------------------|
        | Monday | next time that day exists |
        |--------|---------------------------|
        */

        if let Ok(day) = Date::parse_relative_day(input) {
            return Ok(day);
        }

        if let Ok(month) = Date::parse_relative_month(input) {
            return Ok(month);
        }

        if let Ok(year) = Date::parse_relative_year(input) {
            return Ok(year);
        }

        Err(propagate!(
            DateParser,
            format!("Could not parse relative date '{}'.", input)
        ))
    }

    fn parse_relative_day(input: &str) -> Result<Date, Error> {
        if input.starts_with("in ") {
            if input.ends_with(" day") || input.ends_with(" days") {
                let num = input.split(" ").collect::<Vec<&str>>()[1];
                let offset = match_result!(
                    num.parse::<usize>(),
                    DateParser,
                    format!("Could not parse '{num}' as a number")
                );
                let mut today = Local::now().date_naive();
                today = match_option!(
                    today.checked_add_days(Days::new(match_result!(
                        u64::try_from(offset),
                        DateParser,
                        format!("Could not convert the day offset: {offset}.")
                    ))),
                    DateParser,
                    format!("Could not add the day offset: {offset}.")
                );
                return Ok(match_error!(
                    Date::from_date(today),
                    DateParser,
                    format!("Could not create date object with +{offset} days.")
                ));
            }
        } else if input == "tomorrow" {
            let mut today = Local::now().date_naive();
            today = match_option!(
                today.checked_add_days(Days::new(1)),
                DateParser,
                format!("Could not add an additional day.")
            );
            return Ok(match_error!(
                Date::from_date(today),
                DateParser,
                format!("Could not create date object with +1 day.")
            ));
        }

        if let Ok(day_of_the_week) = Date::parse_day_of_the_week(input) {
            return Ok(day_of_the_week);
        }

        Err(propagate!(
            DateParser,
            format!("Could not parse '{}' into a day", input)
        ))
    }

    fn parse_relative_month(input: &str) -> Result<Date, Error> {
        if input == "" {
            return Err(propagate!(
                DateParser,
                format!("Cannot parse an empty string to a month.")
            ));
        }

        let month = Date::parse_month(input);
        if let Ok(month) = month {
            let current_date = match_error!(
                Date::today(),
                DateParser,
                format!("Could not get today's date.")
            );
            let current_month = current_date.month;
            let mut year = current_date.year;
            if month <= current_month {
                year += 1;
            }

            return Ok(Date {
                day: 1,
                month: month,
                year: year,
            });
        }

        if input.starts_with("in ") {
            if input.ends_with(" month") || input.ends_with(" months") {
                let num = input.split(" ").collect::<Vec<&str>>()[1];
                let offset = match_result!(
                    num.parse::<usize>(),
                    DateParser,
                    format!("Could not parse '{num}' to a number.")
                );
                let mut today = Local::now().date_naive();
                today = match_option!(
                    today.checked_add_months(Months::new(match_result!(
                        u32::try_from(offset),
                        DateParser,
                        format!("Could not convert the month offset: {offset}.")
                    ))),
                    DateParser,
                    format!("Could not add the month offset: {offset}.")
                );

                return Ok(match_error!(
                    Date::from_date(today),
                    DateParser,
                    format!("Could not create date object with +{offset} months.")
                ));
            }
        } else if input == "next month" {
            let mut today = Local::now().date_naive();
            today = match_option!(
                today.checked_add_months(Months::new(1)),
                DateParser,
                format!("Could not add 1 month")
            );
            return Ok(match_error!(
                Date::from_date(today),
                DateParser,
                format!("Could not create date object with +1 month.")
            ));
        }

        Err(propagate!(
            DateParser,
            format!("Could not parse '{}' into a month", input)
        ))
    }

    fn parse_relative_year(input: &str) -> Result<Date, Error> {
        if input.starts_with("in ") {
            if input.ends_with(" year") || input.ends_with(" years") {
                let num = input.split(" ").collect::<Vec<&str>>()[1];
                let offset = match_result!(
                    num.parse::<u16>(),
                    DateParser,
                    format!("Could not parse '{num}' to a number.")
                );
                let mut day = match_error!(
                    Date::today(),
                    DateParser,
                    format!("Could not get the current date.")
                );
                day.year += offset;
                return Ok(day);
            }
        } else if input == "next year" {
            let mut day = match_error!(
                Date::today(),
                DateParser,
                format!("Could not get the current date.")
            );
            day.year += 1;
            return Ok(day);
        }

        Err(propagate!(
            DateParser,
            format!("Could not parse '{}' into a year", input)
        ))
    }

    fn parse_day_of_the_week(input: &str) -> Result<Date, Error> {
        if input == "" {
            return Err(propagate!(
                DateParser,
                format!("Cannot parse empty string to day of")
            ));
        }

        let days = [
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
        ];

        let mut target = -1i32;
        let mut skip_week = false;

        for (i, day) in days.iter().enumerate() {
            let day_of_the_week = match_result!(
                i32::try_from(i),
                DateParser,
                format!("Could not convert day index {i}")
            );

            if day.starts_with(&input.to_string().to_ascii_lowercase()) {
                target = day_of_the_week;
                break;
            } else {
                let input_words = input.split(" ").collect::<Vec<&str>>();
                if input_words.len() == 2
                    && input_words[0] == "next"
                    && day.starts_with(&input_words[1].to_string().to_ascii_lowercase())
                {
                    target = day_of_the_week;
                    skip_week = true;
                    break;
                }
            }
        }

        if target == -1 {
            return Err(propagate!(
                DateParser,
                format!("Day of the week '{}' not recognized", input)
            ));
        }

        let today = match_result!(
            i32::try_from(
                match_error!(
                    match_error!(
                        Date::today(),
                        DateParser,
                        format!("Could not get today's date.")
                    )
                    .as_chrono(),
                    DateParser,
                    format!("Could not convert the date in order to get the weekday.")
                )
                .weekday()
                .num_days_from_monday(),
            ),
            DateParser,
            format!("Could not covnert weekday number")
        );

        let mut delta;

        if target > today {
            // If the day of the week will happen later this week
            delta = target - today;
        } else {
            // If the day is earlier in the week
            delta = target - today + 7
        }

        if skip_week {
            delta += 7;
        }

        // Sanity check
        if delta >= 0 && delta <= 14 {
            let new_date = match_option!(
                match_error!(
                    match_error!(
                        Date::today(),
                        DateParser,
                        format!("Could not get today's date.")
                    )
                    .as_chrono(),
                    DateParser,
                    format!("Could not convert the date in order to get the weekday.")
                )
                .checked_add_days(Days::new(match_result!(
                    u64::try_from(delta),
                    DateParser,
                    format!("Could not convert {delta} days to u64.")
                ))),
                DateParser,
                format!("Could not add the days.")
            );
            println!("abt{delta}");
            return Ok(match_error!(
                Date::from_date(new_date),
                DateParser,
                format!("Could not create date with +{delta} days.")
            ));
        }

        return Err(propagate!(
            DateParser,
            format!("Not implemented '{}'", input)
        ));
    }
}
