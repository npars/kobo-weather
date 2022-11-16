use show_image::{ImageView, ImageInfo, create_window};
use anyhow::Result;
use kobo_weather::render_dashboard;

#[show_image::main]
fn main() -> Result<()> {
    let pixmap = render_dashboard()?;

    let image = ImageView::new(ImageInfo::rgba8(pixmap.width(), pixmap.height()), pixmap.data());

    let window = create_window("Kobo Preview", Default::default())?;
    window.set_image("preview", image)?;

    window.wait_until_destroyed()?;

    Ok(())
}