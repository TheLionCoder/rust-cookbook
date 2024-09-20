use chrono::{DateTime, Duration, FixedOffset, Local, Utc};

pub fn checked_date_and_time() {
    let now: DateTime<Utc> = Utc::now();
    println!("Today: {}", now);

    let almost_three_weeks_from_now: Option<DateTime<Utc>> = now
        .checked_add_signed(Duration::weeks(2))
        .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
        .and_then(get_day_before);

    match almost_three_weeks_from_now {
        Some(datetime) => println!("{}", datetime),
        None => eprintln!("ALmost three weeks from now overflow!"),
    }

    match now.checked_add_signed(Duration::max_value()) {
        Some(datetime) => println!("{}", datetime),
        None => eprintln!("We can't use chrono to tell the time for the Solar System to complete more than one full\
        orbit around the galactic center!"),
    }
}

pub fn convert_local_to_utc() {
    let local_time: DateTime<Local> = Local::now();
    let utc_time: DateTime<Utc> = DateTime::from_naive_utc_and_offset(local_time.naive_utc(), Utc);
    let china_timezone: FixedOffset = FixedOffset::east_opt(8 * 3600).expect("Invalid Offset");
    let rio_timezone: FixedOffset = FixedOffset::west_opt(2 * 3600).expect("Invalid Offset");
    println!("Local time now is {}", local_time);
    println!("Utc time now is {}", utc_time);
    println!(
        "Time in Hong Kong now is {}",
        utc_time.with_timezone(&china_timezone)
    );
    println!(
        "Time in Rio de Janeiro now is {}",
        utc_time.with_timezone(&rio_timezone)
    )
}

// Auxiliar functions
fn get_day_before(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
    date_time.checked_sub_signed(Duration::days(1))
}
