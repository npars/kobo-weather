mod kobo;

use crate::kobo::display;
use anyhow::Result;
use kobo_weather::render_dashboard;

fn main() -> Result<()> {
    let pixmap = render_dashboard()?;

    display(pixmap)?;

    Ok(())
}
