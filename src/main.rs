mod kobo;

use crate::kobo::display;
use anyhow::Result;
use env_logger::Env;
use kobo_weather::render_dashboard;
use log::debug;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("kobo_weather")).init();
    debug!("kobo-weather started");
    let pixmap = render_dashboard()?;

    display(pixmap)?;

    Ok(())
}
