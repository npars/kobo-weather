use crate::weather_icon::WeatherIcon;
use anyhow::{Context, Error, Result};
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
    date_times: Vec<DateTimeResponse>,
    icon_code: Option<String>,
    temperature: f64,
    wind_chill: Option<f64>,
    humidex: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DateTimeResponse {
    zone: String,
    year: String,
    month: NamedFieldResponse,
    day: NamedFieldResponse,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NamedFieldResponse {
    name: String,
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Debug)]
pub(crate) struct WeatherReport {
    pub(crate) current_conditions: CurrentConditions,
    pub(crate) forecast: [Forecast; FORECAST_COUNT],
}

#[derive(Debug)]
pub(crate) struct CurrentConditions {
    pub(crate) day_short_name: String,
    pub(crate) day_long_name: String,
    pub(crate) temp: i32,
    pub(crate) feels_like_temp: Option<i32>,
    pub(crate) weather_icon: WeatherIcon,
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

impl TryFrom<WeatherResponse> for WeatherReport {
    type Error = Error;

    fn try_from(response: WeatherResponse) -> Result<Self> {
        let current_date_time = response
            .current_conditions
            .date_times
            .iter()
            .find(|&date_time| date_time.zone == "AST")
            .context("Could not find current AST time")?;

        Ok(WeatherReport {
            current_conditions: CurrentConditions {
                day_short_name: current_date_time.day.name.to_owned(),
                day_long_name: format!(
                    "{} {}, {}",
                    current_date_time.month.name,
                    current_date_time.day.value,
                    current_date_time.year
                ),
                temp: response.current_conditions.temperature.round() as i32,
                feels_like_temp: response
                    .current_conditions
                    .wind_chill
                    .or(response.current_conditions.humidex)
                    .map(|temp| temp.round() as i32),
                weather_icon: response
                    .current_conditions
                    .icon_code
                    .map_or(WeatherIcon::Na, WeatherIcon::from),
            },
            forecast: [
                Forecast {
                    day_short_name: "".to_string(),
                    low_temp: None,
                    high_temp: None,
                    pop: None,
                    uv: None,
                    weather_icon: WeatherIcon::DaySunny,
                },
                Forecast {
                    day_short_name: "".to_string(),
                    low_temp: None,
                    high_temp: None,
                    pop: None,
                    uv: None,
                    weather_icon: WeatherIcon::DaySunny,
                },
                Forecast {
                    day_short_name: "".to_string(),
                    low_temp: None,
                    high_temp: None,
                    pop: None,
                    uv: None,
                    weather_icon: WeatherIcon::DaySunny,
                },
            ],
        })
    }
}

impl From<String> for WeatherIcon {
    fn from(icon_code: String) -> Self {
        match icon_code.as_str() {
            "00" => WeatherIcon::DaySunny,
            "01" => WeatherIcon::DaySunnyOvercast,
            "02" => WeatherIcon::DayCloudy,
            "03" => WeatherIcon::DayCloudy,
            "04" => WeatherIcon::DayCloudy,
            "05" => WeatherIcon::DayCloudy,
            "06" => WeatherIcon::DayRain,
            "07" => WeatherIcon::DayRainMix,
            "08" => WeatherIcon::DaySnow,
            "09" => WeatherIcon::DayStormShowers,
            "10" => WeatherIcon::Cloudy,
            "11" => WeatherIcon::Sprinkle,
            "12" => WeatherIcon::Showers,
            "13" => WeatherIcon::Rain,
            "14" => WeatherIcon::Hail,
            "15" => WeatherIcon::RainMix,
            "16" => WeatherIcon::Snow,
            "17" => WeatherIcon::Snow,
            "18" => WeatherIcon::Snow,
            "19" => WeatherIcon::StormShowers,
            "20" => WeatherIcon::Cloudy,
            "21" => WeatherIcon::Cloudy,
            "22" => WeatherIcon::DayCloudy,
            "23" => WeatherIcon::Cloudy,
            "24" => WeatherIcon::Cloudy,
            "25" => WeatherIcon::Sandstorm,
            "26" => WeatherIcon::SnowflakeCold,
            "27" => WeatherIcon::Sleet,
            "28" => WeatherIcon::Sprinkle,
            "30" => WeatherIcon::NightClear,
            "31" => WeatherIcon::NightAltCloudy,
            "32" => WeatherIcon::NightAltCloudy,
            "33" => WeatherIcon::NightAltCloudy,
            "34" => WeatherIcon::NightAltCloudy,
            "35" => WeatherIcon::NightAltCloudy,
            "36" => WeatherIcon::NightAltRain,
            "37" => WeatherIcon::NightAltRainMix,
            "38" => WeatherIcon::NightAltSnow,
            "39" => WeatherIcon::NightAltThunderstorm,
            "40" => WeatherIcon::Sandstorm,
            "41" => WeatherIcon::Tornado,
            "42" => WeatherIcon::Tornado,
            "43" => WeatherIcon::StrongWind,
            "44" => WeatherIcon::Fire,
            "45" => WeatherIcon::StrongWind,
            "46" => WeatherIcon::Thunderstorm,
            "47" => WeatherIcon::Lightning,
            "48" => WeatherIcon::Hurricane,
            _ => WeatherIcon::Na,
        }
    }
}

pub(crate) fn fetch_weather() -> Result<WeatherReport> {
    debug!("Fetching weather");
    let body = reqwest::blocking::get(WEATHER_URL)?.text()?;
    let weather_response: WeatherResponse = serde_xml_rs::from_str(&body)?;
    debug!("{:?}", weather_response);

    let report = weather_response.try_into()?;
    debug!("{:?}", report);
    Ok(report)
}
