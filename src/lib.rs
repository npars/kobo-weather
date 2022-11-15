use anyhow::{Context, Result};
use tiny_skia::{ Pixmap, Transform};
use usvg::{FitTo, Options, Tree};

pub fn render(tree: &Tree) -> Result<Pixmap> {
    let size = tree.size.to_screen_size();
    let mut pixmap = Pixmap::new(size.width(), size.height()).context("Failed to create pixmap")?;
    resvg::render(&tree, FitTo::Original, Transform::default(), pixmap.as_mut());
    Ok(pixmap)
}