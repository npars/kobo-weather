use anyhow::{Context, Result};
use log::debug;
use tiny_skia::{Pixmap, Transform};
use usvg::roxmltree::Document;
use usvg::{FitTo, Options, Tree};

const WEATHER_FONT: &[u8] = include_bytes!("../resources/weathericons-regular-webfont.ttf");
const TEXT_FONT: &[u8] = include_bytes!("../resources/Inter-Regular.ttf");

pub(crate) fn render(svg_doc: &Document) -> Result<Pixmap> {
    debug!("Rendering the dashboard");
    let mut opt = Options::default();
    opt.fontdb.load_font_data(WEATHER_FONT.to_vec());
    opt.fontdb.load_font_data(TEXT_FONT.to_vec());

    let tree = Tree::from_xmltree(svg_doc, &opt.to_ref())?;

    let size = tree.size.to_screen_size();
    let mut pixmap = Pixmap::new(size.width(), size.height()).context("Failed to create pixmap")?;
    resvg::render(
        &tree,
        FitTo::Original,
        Transform::default(),
        pixmap.as_mut(),
    );
    Ok(pixmap)
}
