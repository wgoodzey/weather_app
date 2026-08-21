use crate::util::to_12_hour;
use crate::weather::{Daily, Hourly};
use crate::weather_code::WeatherCode;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cell {
    Empty,
    Char(char),
    Continuation,
}

pub struct Canvas {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

fn make_dividers(width: usize, columns: usize) -> Vec<usize> {
    (0..=columns).map(|i| i * (width - 1) / columns).collect()
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![Cell::Empty; width]; height],
            width,
            height,
        }
    }

    fn set_char(&mut self, x: usize, y: usize, ch: char) {
        if y < self.height && x < self.width {
            self.cells[y][x] = Cell::Char(ch);
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

    pub fn paint(&mut self, daily: &Daily, hourly: &Hourly, current_time: &str) {
        self.paint_daily(daily, 0);
        self.paint_hourly(hourly, current_time, 7);
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

        let cols = make_dividers(self.width, n);
        self.draw_dividers(&cols, y0, 6);

        for i in 0..n {
            let left = cols[i] + 1;
            let width = cols[i + 1] - cols[i] - 1;

            let date = format_month_day(&daily.time[i]);

            self.set_str_centered(left, width, y0, &date);
            self.set_icon_centered(left, width, y0 + 1, daily.weather_code[i]);

            self.set_str_centered(
                left,
                width,
                y0 + 2,
                &format!("{:.0}°", daily.temperature_2m_max[i]),
            );

            self.set_str_centered(
                left,
                width,
                y0 + 3,
                &format!("{:.0}°", daily.temperature_2m_min[i]),
            );

            self.set_str_centered(left, width, y0 + 4, &to_12_hour(&daily.sunrise[i]));

            self.set_str_centered(left, width, y0 + 5, &to_12_hour(&daily.sunset[i]));
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

        let cols = make_dividers(self.width, n);
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
            .map(|row| {
                row.iter()
                    .filter_map(|c| match c {
                        Cell::Char(ch) => Some(*ch),
                        Cell::Empty => Some(' '),
                        Cell::Continuation => None,
                    })
                    .collect::<String>()
            })
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
