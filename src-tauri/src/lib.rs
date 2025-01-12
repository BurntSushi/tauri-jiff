use jiff::{Timestamp, ToSpan};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("jiff=trace"))
        .init();

    let time: Timestamp = "2024-07-11T01:14:00Z".parse().unwrap();
    let zoned = time
        .intz("America/New_York")
        .unwrap()
        .checked_add(1.month().hours(2))
        .unwrap();
    println!("##################### actual: {}", zoned);
    println!("##################### expected: 2024-08-10T23:14:00-04:00[America/New_York]");
    println!(
        "##################### system: {:?}",
        jiff::tz::TimeZone::system()
    );

    let now = jiff::Zoned::now();
    println!("##################### now: {now}");
    let rounded = now.round(jiff::Unit::Minute).unwrap();
    println!("##################### now: {rounded}");

    std::process::exit(0);
}
