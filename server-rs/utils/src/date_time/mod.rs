use chrono::{DateTime, Datelike, Local, NaiveDate, TimeZone, Utc};
use error::Error;

pub fn format_datetime(time: &DateTime<Utc>, has_time: bool) -> String {
    if has_time {
        time.with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    } else {
        time.with_timezone(&Local).format("%Y-%m-%d").to_string()
    }
}

pub fn get_month_boundaries(month: &str) -> Result<(DateTime<Utc>, DateTime<Utc>), Error> {
    let parts: Vec<&str> = month.split('/').collect();
    if parts.len() != 2 {
        return Err(Error::InvalidArgument(
            "Invalid month format. Expected 'YYYY/MM'".into(),
        ));
    }

    let year: i32 = parts[0].parse()?;
    let month_num: u32 = parts[1].parse()?;

    let start_of_month = Utc
        .with_ymd_and_hms(year, month_num, 1, 0, 0, 0)
        .single()
        .unwrap();

    let (next_year, next_month) = if month_num == 12 {
        (year + 1, 1)
    } else {
        (year, month_num + 1)
    };

    let start_of_next_month = Utc
        .with_ymd_and_hms(next_year, next_month, 1, 0, 0, 0)
        .single()
        .unwrap();

    Ok((start_of_month, start_of_next_month))
}

pub fn days_in_month(year: u32, month: u32) -> u32 {
    let next_month_date = if month == 12 {
        NaiveDate::from_ymd_opt(year as i32 + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(year as i32, month + 1, 1)
    }
    .expect("Invalid date for next month");

    let last_day_of_current_month = next_month_date
        .pred_opt()
        .expect("Failed to get previous day");

    last_day_of_current_month.day()
}

pub fn generate_days_of_month(year: u32, month: u32, days: u32) -> Vec<String> {
    let mut days_array = Vec::new();
    for day in 1..=days {
        days_array.push(format!("{}-{:02}-{:02}", year, month, day));
    }
    days_array
}

pub fn format_datetime_delimiter(time: &DateTime<Utc>, delimiter: &str, has_time: bool) -> String {
    let full_format = format!("%Y{}%m{}%d %H:%M:%S", delimiter, delimiter);
    let format = format!("%Y{}%m{}%d", delimiter, delimiter);
    if has_time {
        time.with_timezone(&Local).format(&full_format).to_string()
    } else {
        time.with_timezone(&Local).format(&format).to_string()
    }
}

pub fn from_string_to_date(str: &str, fmt: &str) -> Result<DateTime<Utc>, Error> {
    let naive_date = NaiveDate::parse_from_str(str, fmt)
        .map_err(|_| Error::InvalidArgument("请检查日期格式是否正确".to_string()))?;

    let naive_datetime = naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| Error::InvalidArgument("请检查日期是否正确".to_string()))?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
        naive_datetime,
        Utc,
    ))
}

pub fn get_year_month_from_time(time: &DateTime<Utc>) -> (i32, u32) {
    (
        time.with_timezone(&Local).year(),
        time.with_timezone(&Local).month(),
    )
}
