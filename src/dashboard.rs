use anyhow::Result;
use log::debug;
use usvg::roxmltree::Document;

const DASHBOARD_SVG: &str = include_str!("../resources/dashboard.svg");

pub(crate) fn build_dashboard() -> Result<Document<'static>> {
    debug!("Building the dashboard");
    let dashboard = Document::parse(DASHBOARD_SVG)?;
    Ok(dashboard)
}
