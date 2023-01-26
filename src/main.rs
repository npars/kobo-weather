mod kobo;

use crate::kobo::{display, kill_nickel, system_sleep, wifi_down, wifi_up};
use anyhow::Result;
use env_logger::Env;
use kobo_weather::render_dashboard;
use log::error;
use log::info;
use std::time::Duration;

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("kobo_weather")).init();
    info!("kobo-weather started");

    kill_nickel()?;

    if let Err(e) = refresh() {
        error!("Failed to refresh due to error: {}", e)
    }

    if let Err(e) = enter_low_power_mode() {
        error!("Failed to enter low power mode due to error: {}", e)
    }

    if let Err(e) = refresh() {
        error!("Failed to refresh due to error: {}", e)
    }

    Ok(())
}

fn refresh() -> Result<()> {
    let pixmap = render_dashboard()?;
    display(pixmap)?;
    Ok(())
}

fn enter_low_power_mode() -> Result<()> {
    wifi_down()?;
    system_sleep(Duration::from_secs(10))?;
    wifi_up()?;
    Ok(())
}
