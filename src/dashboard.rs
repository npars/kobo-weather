use anyhow::Result;
use usvg::roxmltree::Document;

pub(crate) fn build_dashboard() -> Result<Document<'static>> {
    let dashboard = Document::parse(
        r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
  <rect width="100%" height="100%" fill="white" />
  <text x="100" y="20" font-size="24">Hello World!</text>
</svg>"#,
    )?;

    Ok(dashboard)
}
