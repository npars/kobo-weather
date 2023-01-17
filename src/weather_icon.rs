#[derive(Debug)]
#[allow(dead_code)]
pub(crate) enum WeatherIcon {
    DaySunny,
    DayCloudy,
    DayCloudyGusts,
    DayCloudyWindy,
    DayFog,
    DayHail,
    DayHaze,
    DayLightning,
    DayRain,
    DayRainMix,
    DayRainWind,
    DayShowers,
    DaySleet,
    DaySleetStorm,
    DaySnow,
    DaySnowThunderstorm,
    DaySnowWind,
    DaySprinkle,
    DayStormShowers,
    DaySunnyOvercast,
    DayThunderstorm,
    DayWindy,
    SolarEclipse,
    Hot,
    DayCloudyHigh,
    DayLightWind,
    NightClear,
    NightAltCloudy,
    NightAltCloudyGusts,
    NightAltCloudyWindy,
    NightAltHail,
    NightAltLightning,
    NightAltRain,
    NightAltRainMix,
    NightAltRainWind,
    NightAltShowers,
    NightAltSleet,
    NightAltSleetStorm,
    NightAltSnow,
    NightAltSnowThunderstorm,
    NightAltSnowWind,
    NightAltSprinkle,
    NightAltStormShowers,
    NightAltThunderstorm,
    NightCloudy,
    NightCloudyGusts,
    NightCloudyWindy,
    NightFog,
    NightHail,
    NightLightning,
    NightPartlyCloudy,
    NightRain,
    NightRainMix,
    NightRainWind,
    NightShowers,
    NightSleet,
    NightSleetStorm,
    NightSnow,
    NightSnowThunderstorm,
    NightSnowWind,
    NightSprinkle,
    NightStormShowers,
    NightThunderstorm,
    LunarEclipse,
    Stars,
    StormShowers,
    Thunderstorm,
    NightAltCloudyHigh,
    NightCloudyHigh,
    NightAltPartlyCloudy,
    Cloud,
    Cloudy,
    CloudyGusts,
    CloudyWindy,
    Fog,
    Hail,
    Rain,
    RainMix,
    RainWind,
    Showers,
    Sleet,
    Snow,
    Sprinkle,
    SnowWind,
    Smog,
    Smoke,
    Lightning,
    Raindrops,
    Raindrop,
    Dust,
    SnowflakeCold,
    Windy,
    StrongWind,
    Sandstorm,
    Earthquake,
    Fire,
    Flood,
    Meteor,
    Tsunami,
    Volcano,
    Hurricane,
    Tornado,
    SmallCraftAdvisory,
    GaleWarning,
    StormWarning,
    HurricaneWarning,
    WindDirection,
    Na,
}

impl WeatherIcon {
    pub(crate) fn get_icon_code(&self) -> &str {
        match self {
            Self::DaySunny => "f00d",
            Self::DayCloudy => "f002",
            Self::DayCloudyGusts => "f000",
            Self::DayCloudyWindy => "f001",
            Self::DayFog => "f003",
            Self::DayHail => "f004",
            Self::DayHaze => "f0b6",
            Self::DayLightning => "f005",
            Self::DayRain => "f008",
            Self::DayRainMix => "f006",
            Self::DayRainWind => "f007",
            Self::DayShowers => "f009",
            Self::DaySleet => "f0b2",
            Self::DaySleetStorm => "f068",
            Self::DaySnow => "f00a",
            Self::DaySnowThunderstorm => "f06b",
            Self::DaySnowWind => "f065",
            Self::DaySprinkle => "f00b",
            Self::DayStormShowers => "f00e",
            Self::DaySunnyOvercast => "f00c",
            Self::DayThunderstorm => "f010",
            Self::DayWindy => "f085",
            Self::SolarEclipse => "f06e",
            Self::Hot => "f072",
            Self::DayCloudyHigh => "f07d",
            Self::DayLightWind => "f0c4",
            Self::NightClear => "f02e",
            Self::NightAltCloudy => "f086",
            Self::NightAltCloudyGusts => "f022",
            Self::NightAltCloudyWindy => "f023",
            Self::NightAltHail => "f024",
            Self::NightAltLightning => "f025",
            Self::NightAltRain => "f028",
            Self::NightAltRainMix => "f026",
            Self::NightAltRainWind => "f027",
            Self::NightAltShowers => "f029",
            Self::NightAltSleet => "f0b4",
            Self::NightAltSleetStorm => "f06a",
            Self::NightAltSnow => "f02a",
            Self::NightAltSnowThunderstorm => "f06d",
            Self::NightAltSnowWind => "f067",
            Self::NightAltSprinkle => "f02b",
            Self::NightAltStormShowers => "f02c",
            Self::NightAltThunderstorm => "f02d",
            Self::NightCloudy => "f031",
            Self::NightCloudyGusts => "f02f",
            Self::NightCloudyWindy => "f030",
            Self::NightFog => "f04a",
            Self::NightHail => "f032",
            Self::NightLightning => "f033",
            Self::NightPartlyCloudy => "f083",
            Self::NightRain => "f036",
            Self::NightRainMix => "f034",
            Self::NightRainWind => "f035",
            Self::NightShowers => "f037",
            Self::NightSleet => "f0b3",
            Self::NightSleetStorm => "f069",
            Self::NightSnow => "f038",
            Self::NightSnowThunderstorm => "f06c",
            Self::NightSnowWind => "f066",
            Self::NightSprinkle => "f039",
            Self::NightStormShowers => "f03a",
            Self::NightThunderstorm => "f03b",
            Self::LunarEclipse => "f070",
            Self::Stars => "f077",
            Self::StormShowers => "f01d",
            Self::Thunderstorm => "f01e",
            Self::NightAltCloudyHigh => "f07e",
            Self::NightCloudyHigh => "f080",
            Self::NightAltPartlyCloudy => "f081",
            Self::Cloud => "f041",
            Self::Cloudy => "f013",
            Self::CloudyGusts => "f011",
            Self::CloudyWindy => "f012",
            Self::Fog => "f014",
            Self::Hail => "f015",
            Self::Rain => "f019",
            Self::RainMix => "f017",
            Self::RainWind => "f018",
            Self::Showers => "f01a",
            Self::Sleet => "f0b5",
            Self::Snow => "f01b",
            Self::Sprinkle => "f01c",
            Self::SnowWind => "f064",
            Self::Smog => "f074",
            Self::Smoke => "f062",
            Self::Lightning => "f016",
            Self::Raindrops => "f04e",
            Self::Raindrop => "f078",
            Self::Dust => "f063",
            Self::SnowflakeCold => "f076",
            Self::Windy => "f021",
            Self::StrongWind => "f050",
            Self::Sandstorm => "f082",
            Self::Earthquake => "f0c6",
            Self::Fire => "f0c7",
            Self::Flood => "f07c",
            Self::Meteor => "f071",
            Self::Tsunami => "f0c5",
            Self::Volcano => "f0c8",
            Self::Hurricane => "f073",
            Self::Tornado => "f056",
            Self::SmallCraftAdvisory => "f0cc",
            Self::GaleWarning => "f0cd",
            Self::StormWarning => "f0ce",
            Self::HurricaneWarning => "f0cf",
            Self::WindDirection => "f0b1",
            Self::Na => "f07b",
        }
    }
}
