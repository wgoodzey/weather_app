use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub city: String,
    pub zip: String,
    pub lat: f64,
    pub lon: f64,
}

pub fn fetch(client: &reqwest::blocking::Client) -> reqwest::Result<Location> {
    client.get("http://ip-api.com/json/").send()?.json()
}

fn config_path() -> Option<PathBuf> {
    let home = std::env::var("HOME").ok()?;
    Some(PathBuf::from(home).join(".config/weather/config"))
}

pub fn load_from_config() -> Option<Location> {
    let path = config_path()?;
    let contents = fs::read_to_string(&path).ok()?;

    let mut city = None;
    let mut zip = None;
    let mut lat = None;
    let mut lon = None;

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            match key {
                "city" => city = Some(value.to_string()),
                "zip" => zip = Some(value.to_string()),
                "lat" => lat = value.parse::<f64>().ok(),
                "lon" => lon = value.parse::<f64>().ok(),
                _ => {}
            }
        }
    }

    Some(Location {
        city: city?,
        zip: zip?,
        lat: lat?,
        lon: lon?,
    })
}
