use crate::weather_icon::WeatherIcon;

const FORECAST_COUNT: usize = 3;

struct WeatherReport {
    current_conditions: CurrentConditions,
    forecast: [Forecast; FORECAST_COUNT],
}

struct CurrentConditions {
    day_short_name: String,
    day_long_name: String,
    temp: i8,
    feels_like_temp: i8,
    weather_icon: WeatherIcon,
}

struct Forecast {
    day_short_name: String,
    low_temp: Option<i8>,
    high_temp: Option<i8>,
    pop: Option<u8>,
    uv: Option<u8>,
    weather_icon: WeatherIcon,
}
