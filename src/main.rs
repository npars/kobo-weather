mod lib;

use anyhow::{Context, Result};
use tiny_skia::{ Pixmap, Transform};
use usvg::{FitTo, Options, Tree};
use crate::lib::render;

fn main() -> Result<()> {
    let mut opt = Options::default();
    opt.fontdb.load_system_fonts();

    let tree = Tree::from_str(r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
  <rect width="100%" height="100%" fill="white" />
  <text x="100" y="20" font-size="24">Hello World!</text>
</svg>"#, &opt.to_ref())?;

    let pixmap = render(&tree);

    Ok(())
}
