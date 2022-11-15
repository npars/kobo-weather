use show_image::{ImageView, ImageInfo, create_window};
use anyhow::Result;
use usvg::{Options, Tree};
use kobo_weather::render;

#[show_image::main]
fn main() -> Result<()> {
    let mut opt = Options::default();
    opt.fontdb.load_system_fonts();

    let tree = Tree::from_str(r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
  <rect width="100%" height="100%" fill="white" />
  <text x="100" y="20" font-size="24">Hello World!</text>
</svg>"#, &opt.to_ref())?;

    let pixmap = render(&tree)?;

    let image = ImageView::new(ImageInfo::rgba8(pixmap.width(), pixmap.height()), pixmap.data());

    let window = create_window("Kobo Preview", Default::default())?;
    window.set_image("preview", image)?;

    window.wait_until_destroyed()?;

    Ok(())
}