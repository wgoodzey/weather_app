use crate::location::Location;
use crate::util::to_12_hour;
use crate::weather::{Current, Daily, Hourly};
use crate::weather_code::WeatherCode;

const RESET: &str = "\x1b[0m";
const NO_COLOR: &str = "";

/// Maps a Fahrenheit temperature to an ANSI color escape code.
/// Cold temperatures trend blue/cyan, mild trends green, and hot trends
/// yellow/red/magenta.
fn color_for_temp(temp_f: f64) -> &'static str {
    match temp_f {
        t if t < 32.0 => "\x1b[34m",  // freezing - blue
        t if t < 50.0 => "\x1b[36m",  // cold - cyan
        t if t < 70.0 => "\x1b[32m",  // cool/mild - green
        t if t < 85.0 => "\x1b[33m",  // warm - yellow
        t if t < 100.0 => "\x1b[31m", // hot - red
        _ => "\x1b[35m",              // extreme - magenta
    }
}

pub struct Canvas {
    cells: Vec<Vec<char>>,
    colors: Vec<Vec<&'static str>>,
    width: usize,
    height: usize,
    use_color: bool,
}

fn make_centered_dividers(canvas_width: usize, columns: usize, cell_width: usize) -> Vec<usize> {
    let table_width = columns * (cell_width + 1) + 1;
    let start = canvas_width.saturating_sub(table_width) / 2;

    (0..=columns)
        .map(|i| start + i * (cell_width + 1))
        .collect()
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![' '; width]; height],
            colors: vec![vec![NO_COLOR; width]; height],
            width,
            height,
            use_color: std::env::var_os("NO_COLOR").is_none(),
        }
    }

    fn set_char(&mut self, x: usize, y: usize, ch: char) {
        if y < self.height && x < self.width {
            self.cells[y][x] = ch;
        }
    }

    fn set_char_color(&mut self, x: usize, y: usize, color: &'static str) {
        if y < self.height && x < self.width {
            self.colors[y][x] = color;
        }
    }

    fn set_str(&mut self, x: usize, y: usize, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.set_char(x + i, y, ch);
        }
    }

    fn set_str_colored(&mut self, x: usize, y: usize, s: &str, color: &'static str) {
        for (i, ch) in s.chars().enumerate() {
            self.set_char(x + i, y, ch);
            self.set_char_color(x + i, y, color);
        }
    }

    fn set_str_centered(&mut self, x: usize, width: usize, y: usize, s: &str) {
        let len = s.chars().count();
        let start = if len >= width {
            x
        } else {
            x + (width - len) / 2
        };
        self.set_str(start, y, s);
    }

    fn set_str_centered_colored(
        &mut self,
        x: usize,
        width: usize,
        y: usize,
        s: &str,
        color: &'static str,
    ) {
        let len = s.chars().count();
        let start = if len >= width {
            x
        } else {
            x + (width - len) / 2
        };
        self.set_str_colored(start, y, s, color);
    }

    fn set_icon_centered(&mut self, x: usize, width: usize, y: usize, code: i64) {
        let wc = WeatherCode::from_code(code);
        self.set_str_centered(x, width, y, wc.icon());
    }

    fn draw_dividers(&mut self, xs: &[usize], y0: usize, rows: usize) {
        for &x in xs {
            for y in y0..y0 + rows {
                self.set_char(x, y, '|');
            }
        }
    }

    pub fn paint(
        &mut self,
        location: &Location,
        current: &Current,
        daily: &Daily,
        hourly: &Hourly,
    ) {
        self.paint_header(location, current, 0);
        self.paint_daily(daily, 3);
        self.paint_hourly(hourly, &current.time, 10);
    }

    fn paint_header(&mut self, location: &Location, current: &Current, y0: usize) {
        let location_line = format!("{}, {}", location.city, location.zip);
        self.set_str_centered(0, self.width, y0, &location_line);

        let weather = WeatherCode::from_code(current.weather_code).icon();
        let temp_str = format!("{:.0}°F", current.temperature_2m);
        let feels_str = format!("{:.0}°F", current.apparent_temperature);
        let precip_str = format!("{:.2} in", current.precipitation);

        let sep1 = "  ";
        let sep2 = "  ·  FEELS ";
        let sep3 = "  ·  PRECIP ";

        let full_len = weather.chars().count()
            + sep1.chars().count()
            + temp_str.chars().count()
            + sep2.chars().count()
            + feels_str.chars().count()
            + sep3.chars().count()
            + precip_str.chars().count();

        let mut x = if full_len >= self.width {
            0
        } else {
            (self.width - full_len) / 2
        };
        let y = y0 + 1;

        self.set_str(x, y, weather);
        x += weather.chars().count();

        self.set_str(x, y, sep1);
        x += sep1.chars().count();

        self.set_str_colored(x, y, &temp_str, color_for_temp(current.temperature_2m));
        x += temp_str.chars().count();

        self.set_str(x, y, sep2);
        x += sep2.chars().count();

        self.set_str_colored(
            x,
            y,
            &feels_str,
            color_for_temp(current.apparent_temperature),
        );
        x += feels_str.chars().count();

        self.set_str(x, y, sep3);
        x += sep3.chars().count();

        self.set_str(x, y, &precip_str);
    }

    fn paint_daily(&mut self, daily: &Daily, y0: usize) {
        let n = daily
            .time
            .len()
            .min(daily.weather_code.len())
            .min(daily.temperature_2m_max.len())
            .min(daily.temperature_2m_min.len())
            .min(daily.sunrise.len())
            .min(daily.sunset.len())
            .min(6);

        if n == 0 {
            return;
        }

        const DAILY_CELL_WIDTH: usize = 9;
        let cols = make_centered_dividers(self.width, n, DAILY_CELL_WIDTH);
        self.draw_dividers(&cols, y0, 6);

        let data_y = y0;

        for i in 0..n {
            let left = cols[i] + 1;
            let width = cols[i + 1] - cols[i] - 1;

            let date = format_month_day(&daily.time[i]);

            self.set_str_centered(left, width, data_y, &date);
            self.set_icon_centered(left, width, data_y + 1, daily.weather_code[i]);

            self.set_str_centered_colored(
                left,
                width,
                data_y + 2,
                &format!("{:.0}°", daily.temperature_2m_max[i]),
                color_for_temp(daily.temperature_2m_max[i]),
            );

            self.set_str_centered_colored(
                left,
                width,
                data_y + 3,
                &format!("{:.0}°", daily.temperature_2m_min[i]),
                color_for_temp(daily.temperature_2m_min[i]),
            );

            self.set_str_centered(left, width, data_y + 4, &to_12_hour(&daily.sunrise[i]));
            self.set_str_centered(left, width, data_y + 5, &to_12_hour(&daily.sunset[i]));
        }
    }

    fn paint_hourly(&mut self, hourly: &Hourly, current_time: &str, y0: usize) {
        let start = hourly
            .time
            .iter()
            .position(|t| t.as_str() > current_time)
            .unwrap_or(0);

        let available = hourly
            .time
            .len()
            .min(hourly.weather_code.len())
            .min(hourly.temperature_2m.len())
            .min(hourly.precipitation_probability.len());

        let n = available.saturating_sub(start).min(9);

        if n == 0 {
            return;
        }

        const HOURLY_CELL_WIDTH: usize = 7;
        let cols = make_centered_dividers(self.width, n, HOURLY_CELL_WIDTH);
        self.draw_dividers(&cols, y0, 4);

        for i in 0..n {
            let idx = start + i;
            let left = cols[i] + 1;
            let width = cols[i + 1] - cols[i] - 1;

            let time = hourly.time[idx].split('T').nth(1).unwrap_or("");

            self.set_str_centered(left, width, y0, time);
            self.set_icon_centered(left, width, y0 + 1, hourly.weather_code[idx]);

            self.set_str_centered_colored(
                left,
                width,
                y0 + 2,
                &format!("{:.0}°", hourly.temperature_2m[idx]),
                color_for_temp(hourly.temperature_2m[idx]),
            );

            self.set_str_centered(
                left,
                width,
                y0 + 3,
                &format!("{}%", hourly.precipitation_probability[idx]),
            );
        }
    }

    pub fn render(&self) -> String {
        let mut out = String::with_capacity((self.width + 1) * self.height);

        for y in 0..self.height {
            let mut active = NO_COLOR;

            for x in 0..self.width {
                let color = if self.use_color {
                    self.colors[y][x]
                } else {
                    NO_COLOR
                };

                if color != active {
                    if !active.is_empty() {
                        out.push_str(RESET);
                    }
                    if !color.is_empty() {
                        out.push_str(color);
                    }
                    active = color;
                }

                out.push(self.cells[y][x]);
            }

            if !active.is_empty() {
                out.push_str(RESET);
            }

            if y + 1 < self.height {
                out.push('\n');
            }
        }

        out
    }

    pub fn print(&self) {
        println!("{}", self.render());
    }
}

fn format_month_day(iso_date: &str) -> String {
    let mut parts = iso_date.splitn(3, '-');
    let _year = parts.next();
    let month = parts.next().unwrap_or("00");
    let day = parts.next().unwrap_or("00");
    return format!("{month}/{day}");
}