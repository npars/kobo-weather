use anyhow::Result;
use env_logger::Env;
use kobo_weather::render_dashboard;
use show_image::{create_window, ImageInfo, ImageView};

#[show_image::main]
fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("kobo_weather")).init();

    let pixmap = render_dashboard()?;

    let image = ImageView::new(
        ImageInfo::rgba8(pixmap.width(), pixmap.height()),
        pixmap.data(),
    );

    let window = create_window("Kobo Preview", Default::default())?;
    window.set_image("preview", image)?;

    window.wait_until_destroyed()?;

    Ok(())
}
