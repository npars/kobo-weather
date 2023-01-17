use crate::weather::{Forecast, WeatherReport};

use log::debug;

const DASHBOARD_SVG: &str = include_str!("../resources/dashboard.svg");

pub(crate) fn build_dashboard(weather: WeatherReport) -> String {
    debug!("Building the dashboard");
    let mut dashboard = update_current_day(&weather, DASHBOARD_SVG);
    for (index, forecast) in weather.forecast.iter().enumerate() {
        dashboard = update_forecast(forecast, index, &dashboard);
    }
    dashboard
}

fn update_current_day(weather: &WeatherReport, svg: &str) -> String {
    svg.replace(
        "$$current-day-short-name$$",
        &weather.current_conditions.day_short_name,
    )
    .replace(
        "$$current-day-long-name$$",
        &weather.current_conditions.day_long_name,
    )
    .replace(
        "$$current-day-icon$$",
        &format!(
            "&#x{};",
            weather.current_conditions.weather_icon.get_icon_code()
        ),
    )
    .replace(
        "$$current-day-temp$$",
        &weather.current_conditions.temp.to_string(),
    )
    .replace(
        "$$current-day-feel-visibility$$",
        get_visibility(weather.current_conditions.feels_like_temp),
    )
    .replace(
        "$$current-day-feel$$",
        &weather
            .current_conditions
            .feels_like_temp
            .map_or("-".to_string(), |temp| temp.to_string()),
    )
}

fn update_forecast(forecast: &Forecast, index: usize, svg: &str) -> String {
    svg.replace(
        &format!("$$forecast{index}-icon$$"),
        &format!("&#x{};", forecast.weather_icon.get_icon_code()),
    )
    .replace(
        &format!("$$forecast{index}-short-name$$"),
        &forecast.day_short_name,
    )
}

fn get_visibility<T>(optional: Option<T>) -> &'static str {
    match optional {
        Some(_) => "visible",
        None => "hidden",
    }
}
