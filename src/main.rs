use anyhow::{Context, Result};
use resvg::render;
use tiny_skia::{ Pixmap, Transform};
use usvg::{FitTo, Options, Tree};

fn main() -> Result<()> {
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();

    let tree = Tree::from_str(r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
  <rect width="100%" height="100%" fill="white" />
  <text x="100" y="20" font-size="24">Hello World!</text>
</svg>"#, &opt.to_ref()).unwrap();

    let mut pixmap = Pixmap::new(800, 600).context("Failed to create pixmap")?;
    render(&tree, FitTo::Original, Transform::default(), pixmap.as_mut());

    pixmap.save_png("/Users/nparsons/Desktop/out.png")?;
    Ok(())
}
