use crate::{util::to_12_hour, weather_code::WeatherCode};

mod location;
mod util;
mod weather;
mod weather_code;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let loc = location::fetch(&client)?;
    println!("City: {}", loc.city);
    println!("Zip Code: {}", loc.zip);

    let num_days: i8 = 3; // 1-7

    let w = weather::fetch(&client, loc.lat, loc.lon, num_days)?;

    println!(
        "Current Temperature: {}ºF, feels like: {}ºF, conditions: {} {}",
        w.current.temperature_2m,
        w.current.apparent_temperature,
        WeatherCode::from_code(w.current.weather_code).label(),
        WeatherCode::from_code(w.current.weather_code).icon()
    );

    println!("Next {} days", w.daily.time.len());

    for ((day, sunrise), sunset) in w
        .daily
        .time
        .iter()
        .zip(w.daily.sunrise.iter())
        .zip(w.daily.sunset.iter())
    {
        println!("{day}: {} - {}", to_12_hour(sunrise), to_12_hour(sunset));
    }

    Ok(())
}
