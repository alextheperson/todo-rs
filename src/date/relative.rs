use chrono::Datelike;
use chrono::Days;
use chrono::Months;
use chrono::offset::Local;

use crate::date::Date;

impl Date {
    pub fn parse_relative_date(input: &str) -> Result<Date, String> {
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

        let day = Date::parse_relative_day(input);
        if day.is_ok() {
            return Ok(day.unwrap());
        }

        let month = Date::parse_relative_month(input);
        if month.is_ok() {
            return Ok(month.unwrap());
        }

        let month = Date::parse_relative_year(input);
        if month.is_ok() {
            return Ok(month.unwrap());
        }

        Err(format!("Could not parse relative date '{}'.", input))
    }

    fn parse_relative_day(input: &str) -> Result<Date, String> {
        if input.starts_with("in ") {
            if input.ends_with(" day") || input.ends_with(" days") {
                let offset = input.split(" ").collect::<Vec<&str>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let mut today = Local::now().date_naive();
                today = today
                    .checked_add_days(Days::new(u64::try_from(offset).unwrap()))
                    .unwrap();
                return Ok(Date::from_date(today));
            }
        } else if input == "tomorrow" {
            let mut today = Local::now().date_naive();
            today = today.checked_add_days(Days::new(1)).unwrap();
            return Ok(Date::from_date(today));
        }

        let day_of_the_week = Date::parse_day_of_the_week(input);
        if day_of_the_week.is_ok() {
            return Ok(day_of_the_week.unwrap());
        }

        Err(format!("Could not parse '{}' into a day", input))
    }

    fn parse_relative_month(input: &str) -> Result<Date, String> {
        if input.starts_with("in ") {
            if input.ends_with(" month") || input.ends_with(" months") {
                let offset = input.split(" ").collect::<Vec<&str>>()[1]
                    .parse::<usize>()
                    .unwrap();
                let mut today = Local::now().date_naive();
                today = today
                    .checked_add_months(Months::new(u32::try_from(offset).unwrap()))
                    .unwrap();
                return Ok(Date::from_date(today));
            }
        } else if input == "next month" {
            let mut today = Local::now().date_naive();
            today = today.checked_add_months(Months::new(1)).unwrap();
            return Ok(Date::from_date(today));
        }

        Err(format!("Could not parse '{}' into a month", input))
    }

    fn parse_relative_year(input: &str) -> Result<Date, String> {
        if input.starts_with("in ") {
            if input.ends_with(" year") || input.ends_with(" years") {
                let offset = input.split(" ").collect::<Vec<&str>>()[1]
                    .parse::<u16>()
                    .unwrap();
                let mut day = Date::today();
                day.year += offset;
                return Ok(day);
            }
        } else if input == "next year" {
            let mut day = Date::today();
            day.year += 1;
            return Ok(day);
        }

        Err(format!("Could not parse '{}' into a year", input))
    }

    fn parse_day_of_the_week(input: &str) -> Result<Date, String> {
        if input == "" {
            return Err(format!("Cannot parse empty string to day of the week."));
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
            if day.starts_with(&input.to_string().to_ascii_lowercase()) {
                target = i32::try_from(i).unwrap();
                break;
            } else {
                let input_words = input.split(" ").collect::<Vec<&str>>();
                if input_words.len() == 2 {}
                if input_words.len() == 2
                    && input_words[0] == "next"
                    && day.starts_with(&input_words[1].to_string().to_ascii_lowercase())
                {
                    target = i32::try_from(i).unwrap();
                    skip_week = true;
                    break;
                }
            }
        }

        if target == -1 {
            return Err(format!("Day of the week '{}' not recognized", input));
        }

        let today = i32::try_from(
            Date::today()
                .as_chrono()
                .unwrap()
                .weekday()
                .num_days_from_monday(),
        )
        .unwrap();

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
            let new_date = Date::today()
                .as_chrono()
                .unwrap()
                .checked_add_days(Days::new(u64::try_from(delta).unwrap()))
                .unwrap();
            return Ok(Date::from_date(new_date));
        }

        return Err(format!("Not implemented '{}'", input));
    }
}
