use crate::location::Location;
use crate::util::to_12_hour;
use crate::weather::{Current, Daily, Hourly};
use crate::weather_code::WeatherCode;

pub struct Canvas {
    cells: Vec<Vec<char>>,
    width: usize,
    height: usize,
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
            width,
            height,
        }
    }

    fn set_char(&mut self, x: usize, y: usize, ch: char) {
        if y < self.height && x < self.width {
            self.cells[y][x] = ch;
        }
    }

    fn set_str(&mut self, x: usize, y: usize, s: &str) {
        for (i, ch) in s.chars().enumerate() {
            self.set_char(x + i, y, ch);
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
        let weather = WeatherCode::from_code(current.weather_code).icon();
        let current_line = format!(
            "{weather}  {:.0}°F  ·  FEELS {:.0}°F  ·  PRECIP {:.2} in",
            current.temperature_2m, current.apparent_temperature, current.precipitation,
        );

        self.set_str_centered(0, self.width, y0, &location_line);
        self.set_str_centered(0, self.width, y0 + 1, &current_line);
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

            self.set_str_centered(
                left,
                width,
                data_y + 2,
                &format!("{:.0}°", daily.temperature_2m_max[i]),
            );

            self.set_str_centered(
                left,
                width,
                data_y + 3,
                &format!("{:.0}°", daily.temperature_2m_min[i]),
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

            self.set_str_centered(
                left,
                width,
                y0 + 2,
                &format!("{:.0}°", hourly.temperature_2m[idx]),
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
        self.cells
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
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
    format!("{month}/{day}")
}
