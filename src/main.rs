mod location;
mod weather;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let loc = location::fetch(&client)?;
    println!("City: {}", loc.city);
    println!("Zip Code: {}", loc.zip);

    let w = weather::fetch(&client, loc.lat, loc.lon)?;
    println!("Current Temperature: {}ºF, feels like: {}ºF", w.current.temperature_2m, w.current.apparent_temperature);

    Ok(())
}
