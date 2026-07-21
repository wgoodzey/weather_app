use serde_json::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let location: Value = client
        .get("http://ip-api.com/json/")
        .send()?
        .json()?;

    let city = location["city"].as_str().unwrap();
    let zip = location["zip"].as_str().unwrap();
    let lon = location["lon"].as_f64().unwrap();
    let lat = location["lat"].as_f64().unwrap();

    println!("City: {}", city);
    println!("Zip Code: {}", zip);

    let weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m",
        lat, lon
    );

    let weather: Value = client.get(&weather_url).send()?.json()?;
    let temp = weather["current"]["temperature_2m"].as_f64().unwrap();

    println!("Current Temperature: {}", temp);

    Ok(())
}
