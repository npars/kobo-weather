mod dashboard;
mod render;

use crate::dashboard::build_dashboard;
use crate::render::render;
use anyhow::Result;
use tiny_skia::Pixmap;

pub fn render_dashboard() -> Result<Pixmap> {
    let dash_svg = build_dashboard()?;
    render(&dash_svg)
}
