#[derive(Debug, PartialEq, Clone, Copy)]
pub enum WeatherCode {
    ClearSky,             // 0
    MainlyClear,          // 1
    PartlyCloudy,         // 2
    Overcast,             // 3
    Fog,                  // 45
    DepositingRimeFog,    // 48
    DrizzleLight,         // 51
    DrizzleModerate,      // 53
    DrizzleDense,         // 55
    FreezingDrizzleLight, // 56
    FreezingDrizzleDense, // 57
    RainSlight,           // 61
    RainModerate,         // 63
    RainHeavy,            // 65
    FreezingRainLight,    // 66
    FreezingRainHeavy,    // 67
    SnowSlight,           // 71
    SnowModerate,         // 73
    SnowHeavy,            // 75
    SnowGrains,           // 77
    RainShowersSlight,    // 80
    RainShowersModerate,  // 81
    RainShowersViolent,   // 82
    SnowShowersSlight,    // 85
    SnowShowersHeavy,     // 86
    ThunderstormSlight,   // 95
    ThunderstormHail,     // 96 or 99
    Unknown,              // fallback
}

impl WeatherCode {
    pub fn from_code(code: i64) -> Self {
        use WeatherCode::*;
        match code {
            0 => ClearSky,
            1 => MainlyClear,
            2 => PartlyCloudy,
            3 => Overcast,
            45 => Fog,
            48 => DepositingRimeFog,
            51 => DrizzleLight,
            53 => DrizzleModerate,
            55 => DrizzleDense,
            56 => FreezingDrizzleLight,
            57 => FreezingDrizzleDense,
            61 => RainSlight,
            63 => RainModerate,
            65 => RainHeavy,
            66 => FreezingRainLight,
            67 => FreezingRainHeavy,
            71 => SnowSlight,
            73 => SnowModerate,
            75 => SnowHeavy,
            77 => SnowGrains,
            80 => RainShowersSlight,
            81 => RainShowersModerate,
            82 => RainShowersViolent,
            85 => SnowShowersSlight,
            86 => SnowShowersHeavy,
            95 => ThunderstormSlight,
            96 | 99 => ThunderstormHail,
            _ => Unknown,
        }
    }

    pub fn icon(&self) -> &'static str {
        use WeatherCode::*;
        match self {
            ClearSky => "☀",
            MainlyClear | PartlyCloudy => "🌤",
            Overcast => "☁",
            Fog | DepositingRimeFog => "🌫",
            DrizzleLight | DrizzleModerate | DrizzleDense => "🌦",
            FreezingDrizzleLight | FreezingDrizzleDense => "🌧",
            RainSlight | RainModerate | RainHeavy => "🌧",
            FreezingRainLight | FreezingRainHeavy => "❅",
            SnowSlight | SnowModerate | SnowHeavy | SnowGrains => "❄",
            RainShowersSlight | RainShowersModerate | RainShowersViolent => "🌦",
            SnowShowersSlight | SnowShowersHeavy => "🌨",
            ThunderstormSlight | ThunderstormHail => "⛈",
            Unknown => "?",
        }
    }

    pub fn label(&self) -> &'static str {
        use WeatherCode::*;
        match self {
            ClearSky => "Clear sky",
            MainlyClear => "Mainly clear",
            PartlyCloudy => "Partly cloudy",
            Overcast => "Overcast",
            Fog => "Fog",
            DepositingRimeFog => "Rime fog",
            DrizzleLight => "Light drizzle",
            DrizzleModerate => "Moderate drizzle",
            DrizzleDense => "Dense drizzle",
            FreezingDrizzleLight => "Light freezing drizzle",
            FreezingDrizzleDense => "Dense freezing drizzle",
            RainSlight => "Slight rain",
            RainModerate => "Moderate rain",
            RainHeavy => "Heavy rain",
            FreezingRainLight => "Light freezing rain",
            FreezingRainHeavy => "Heavy freezing rain",
            SnowSlight => "Slight snow",
            SnowModerate => "Moderate snow",
            SnowHeavy => "Heavy snow",
            SnowGrains => "Snow grains",
            RainShowersSlight => "Slight rain showers",
            RainShowersModerate => "Moderate rain showers",
            RainShowersViolent => "Violent rain showers",
            SnowShowersSlight => "Slight snow showers",
            SnowShowersHeavy => "Heavy snow showers",
            ThunderstormSlight => "Thunderstorm",
            ThunderstormHail => "Thunderstorm with hail",
            Unknown => "Unknown",
        }
    }
}
