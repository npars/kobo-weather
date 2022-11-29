use crate::weather_icon::WeatherIcon;
use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

const FORECAST_COUNT: usize = 3;
const WEATHER_URL: &str = "https://dd.weather.gc.ca/citypage_weather/xml/NS/s0000318_e.xml";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct WeatherResponse {
    current_conditions: CurrentConditionsResponse,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct CurrentConditionsResponse {
    #[serde(rename = "dateTime")]
    date_times: Vec<DateTime>,
    icon_code: Option<String>,
    temperature: f64,
    wind_chill: Option<f64>,
    humidex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DateTime {
    zone: String,
    year: String,
    month: NamedField,
    day: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NamedField {
    name: String,
}

#[derive(Debug)]
pub(crate) struct WeatherReport {
    current_conditions: CurrentConditions,
    forecast: [Forecast; FORECAST_COUNT],
}

#[derive(Debug)]
pub(crate) struct CurrentConditions {
    day_short_name: String,
    day_long_name: String,
    temp: f64,
    feels_like_temp: f64,
    weather_icon: WeatherIcon,
}

#[derive(Debug)]
pub(crate) struct Forecast {
    day_short_name: String,
    low_temp: Option<f64>,
    high_temp: Option<f64>,
    pop: Option<u8>,
    uv: Option<u8>,
    weather_icon: WeatherIcon,
}

pub(crate) fn fetch_weather() -> Result<()> {
    debug!("Fetching weather");
    let body = reqwest::blocking::get(WEATHER_URL)?.text()?;
    let weather_response: WeatherResponse = serde_xml_rs::from_str(&body)?;

    debug!("{:?}", weather_response);
    Ok(())
}
