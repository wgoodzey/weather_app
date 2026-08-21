mod location;
mod render;
mod util;
mod weather;
mod weather_code;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

    let loc = location::fetch(&client)?;

    let num_days: i8 = 6; // 1-7
    let w = weather::fetch(&client, loc.lat, loc.lon, num_days)?;

    let mut canvas = render::Canvas::new(80, 14);
    canvas.paint(&loc, &w.current, &w.daily, &w.hourly);
    canvas.print();

    Ok(())
}
