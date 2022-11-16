mod dashboard;
mod render;

use anyhow::Result;
use tiny_skia::Pixmap;
use crate::dashboard::build_dashboard;
use crate::render::render;

pub fn render_dashboard() -> Result<Pixmap> {
    let dash_svg = build_dashboard()?;
    let dash_pixmap = render(&dash_svg)?;

    Ok(dash_pixmap)
}