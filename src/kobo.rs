use anyhow::Result;
use tiny_skia::Pixmap;

fn to_rgb565<'a>(image: Pixmap) -> Result<Vec<u8>> {
    let rgb565 = image
        .data()
        .chunks(4)
        .flat_map(|chunk| {
            if let [r, g, b, alpha] = chunk {
                rgb888_to_rgb565(*r, *g, *b, *alpha)
            } else {
                panic!("Invalid image size!")
            }
        })
        .collect::<Vec<u8>>();

    Ok(rgb565)
}

fn rgb888_to_rgb565(r: u8, g: u8, b: u8, _alpha: u8) -> [u8; 2] {
    let r5 = (r >> 3) as u16;
    let g6 = (g >> 2) as u16;
    let b5 = (b >> 3) as u16;

    let word = (r5 << 11) | (g6 << 5) | b5;
    word.to_le_bytes()
}

#[cfg(test)]
mod tests {
    use crate::kobo::to_rgb565;
    use anyhow::{Context, Result};
    use tiny_skia::Pixmap;
    use tiny_skia_path::IntSize;

    #[test]
    fn converts_all_white_image() -> Result<()> {
        let pixmap = Pixmap::from_vec(
            vec![0xff, 0xff, 0xff, 0xff],
            IntSize::from_wh(1, 1).context("Bad size")?,
        )
        .context("Bad Vec")?;
        let result = to_rgb565(pixmap)?;
        assert_eq!(result, vec!(0xff, 0xff));
        Ok(())
    }
}
