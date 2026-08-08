use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub precipitation: Vec<f64>,
    pub weather_code: Vec<i64>,
}

#[derive(Deserialize, Debug)]
pub struct Daily {
    pub time: Vec<String>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Current {
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub weather_code: i64,
    pub precipitation: f64,
    pub is_day: i64,
}

#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    pub current: Current,
    pub hourly: Hourly,
    pub daily: Daily,
}

pub fn fetch(
    client: &reqwest::blocking::Client,
    lat: f64,
    lon: f64,
    days: i8,
) -> reqwest::Result<WeatherResponse> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={lon}&daily=sunrise,sunset&hourly=temperature_2m,precipitation,weather_code&models=dwd_icon_seamless&current=weather_code,temperature_2m,apparent_temperature,precipitation,is_day&timezone=auto&forecast_days={days}&wind_speed_unit=mph&temperature_unit=fahrenheit&precipitation_unit=inch"
    );
    client.get(&url).send()?.json()
}
