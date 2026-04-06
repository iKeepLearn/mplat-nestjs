use chrono::{DateTime, Local, NaiveDate, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

const FORMAT: &str = "%Y-%m-%d";

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.with_timezone(&Local).format(FORMAT));
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let naive_date = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;

    let naive_datetime = naive_date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| serde::de::Error::custom("Invalid time"))?;

    Ok(DateTime::<Utc>::from_naive_utc_and_offset(
        naive_datetime,
        Utc,
    ))
}
