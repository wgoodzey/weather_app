use serde::Deserialize;

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
