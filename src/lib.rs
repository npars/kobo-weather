mod dashboard;
mod render;
mod weather;
mod weather_icon;

use crate::dashboard::build_dashboard;
use crate::render::render;
use crate::weather::fetch_weather;
use anyhow::Result;
use tiny_skia::Pixmap;

pub fn render_dashboard() -> Result<Pixmap> {
    fetch_weather()?;
    let dash_svg = build_dashboard()?;
    render(&dash_svg)
}
