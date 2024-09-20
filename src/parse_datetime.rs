use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};
use chrono::format::ParseError;

pub fn examine_date_and_time() {
    let now: DateTime<Utc> = Utc::now();

    let (is_pm, hour) = now.hour12();
    println!(
        "The current UTC time is {:02}:{:02}:{:02} {}",
        hour,
        now.minute(),
        now.second(),
        if is_pm { "PM" } else { "AM" }
    );

    let (is_common_era, year) = now.year_ce();
    println!(
        "The current UTC date is {}-{:02}-{:02} {:?} ({})",
        year,
        now.month(),
        now.day(),
        now.weekday(),
        if is_common_era { "CE" } else { "BCE" }
    );
    println!(
        "And the common Era began {} days ago",
        now.num_days_from_ce()
    )
}

pub fn convert_date_to_unix() {
    let date_time: NaiveDateTime = NaiveDate::from_ymd_opt(2017, 11, 12)
        .unwrap()
        .and_hms_opt(17, 33, 44).unwrap();
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}",
        date_time, date_time.and_utc().timestamp()
    );

    let date_time_after_a_billion_seconds:  DateTime<Utc> = DateTime::from_timestamp(1_000_000_000, 0).unwrap();
    println!(
        "Date after a billions seconds since 1970-01-01 00:00:00 was {}",
        date_time_after_a_billion_seconds
    );
}

pub fn display_formatted_date_time() {
    let now: DateTime<Utc> = Utc::now();

    println!("UTC now is :{}", now);
    println!("UTC now in RFC 2822 is: {}", now.to_rfc2822());
    println!("UTC now in RFC 3389 is: {}", now.to_rfc3339());
    println!("UTC now in a custom format is: {}", now.format("%a %b %e %T %Y"));
}

pub fn parse_string_into_datetime_struct() -> Result<(), ParseError> {
    let rfc2822: DateTime<FixedOffset>= DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200")?;
    println!("{}", rfc2822);

    let rfc3339: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("1996-12-19T16:39:57-08:00")?;
    println!("{}", rfc3339);

    let custom_format: DateTime<FixedOffset> = DateTime::parse_from_str("5.8.1994 8:00 am +0000",
                                                                        "%d.%m.%Y %H:%M %P %z")?;
    println!("{}", custom_format);

    let time_only: NaiveTime = NaiveTime::parse_from_str("23:56:04", "%H:%M:%S")?;
    println!("{}", time_only);

    let date_only: NaiveDate = NaiveDate::parse_from_str("2015-09-05", "%Y-%m-%d")?;
    println!("{}", date_only);

    let not_time_zone: NaiveDateTime = NaiveDateTime::parse_from_str("2015-09-05 23:56:04", "%Y-%m-%d %H:%M:%S")?;
    println!("{}", not_time_zone);

    Ok(())
}
