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

const DAILY_COLS: [usize; 7] = [7, 19, 29, 39, 49, 59, 69];

const HOURLY_COLS: [usize; 10] = [2, 10, 18, 26, 34, 42, 50, 58, 66, 74];

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

    fn set_wide_char(&mut self, x: usize, y: usize, ch: char) {
        self.set_char(x, y, ch);
        if x + 1 < self.width && y < self.height {
            self.cells[y][x + 1] = Cell::Continuation;
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
        let iw = wc.icon_width();
        let Some(ch) = wc.icon().chars().next() else {
            return;
        };
        let icon_x = x + width.saturating_sub(iw) / 2;
        if iw == 2 {
            self.set_wide_char(icon_x, y, ch);
        } else {
            self.set_char(icon_x, y, ch);
        }
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
        self.paint_hourly(hourly, current_time, 9);
    }

    fn paint_daily(&mut self, daily: &Daily, y0: usize) {
        self.draw_dividers(&DAILY_COLS, y0, 8);

        let n = daily
            .time
            .len()
            .min(daily.weather_code.len())
            .min(daily.temperature_2m_max.len())
            .min(daily.temperature_2m_min.len())
            .min(daily.sunrise.len())
            .min(daily.sunset.len())
            .min(DAILY_COLS.len() - 1);

        for i in 0..n {
            let left = DAILY_COLS[i] + 1;
            let width = DAILY_COLS[i + 1] - DAILY_COLS[i] - 1;

            let date = format_month_day(&daily.time[i]);
            self.set_str_centered(left, width, y0 + 1, &date);

            self.set_icon_centered(left, width, y0 + 2, daily.weather_code[i]);

            self.set_str_centered(
                left,
                width,
                y0 + 3,
                &format!("{:.0}°", daily.temperature_2m_max[i]),
            );
            self.set_str_centered(
                left,
                width,
                y0 + 4,
                &format!("{:.0}°", daily.temperature_2m_min[i]),
            );

            self.set_str_centered(left, width, y0 + 5, &to_12_hour(&daily.sunrise[i]));
            self.set_str_centered(left, width, y0 + 6, &to_12_hour(&daily.sunset[i]));
        }
    }

    fn paint_hourly(&mut self, hourly: &Hourly, current_time: &str, y0: usize) {
        self.draw_dividers(&HOURLY_COLS, y0, 4);

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

        let n = available.saturating_sub(start).min(HOURLY_COLS.len() - 1);

        for i in 0..n {
            let idx = start + i;
            let left = HOURLY_COLS[i] + 1;
            let width = HOURLY_COLS[i + 1] - HOURLY_COLS[i] - 1;

            let time = hourly.time[idx].split('T').nth(1).unwrap_or("").to_string();
            self.set_str_centered(left, width, y0, &time);

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
