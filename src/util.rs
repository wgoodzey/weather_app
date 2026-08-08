pub fn to_12_hour(time_24: &str) -> String {
    let time_part = time_24.split('T').nth(1).unwrap_or(time_24);

    let mut parts = time_part.splitn(2, ':');
    let hour: u8 = parts.next().unwrap_or("0").parse().unwrap_or(0);
    let minutes = parts.next().unwrap_or("0");
    let period = if hour >= 12 { "PM" } else { "AM" };

    let hour_12 = match hour % 12 {
        0 => 12,
        h => h,
    };

    format!("{hour_12}:{minutes} {period}")
}
