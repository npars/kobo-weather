use anyhow::{Context, Result};
use log::debug;
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};
use tiny_skia::Pixmap;

pub fn display(image: Pixmap) -> Result<()> {
    debug!("Displaying the dashboard");
    let raw_image = to_rgb565_le(&image);
    display_image(raw_image)
}

fn display_image(raw_image: impl Iterator<Item = u8>) -> Result<()> {
    let mut pickel = Command::new("/usr/local/Kobo/pickel")
        .arg("showpic")
        .stdin(Stdio::piped())
        .spawn()?;

    {
        let mut buffer = BufWriter::new(pickel.stdin.as_ref().context("Failed to pipe to pickel")?);
        raw_image.for_each(|byte| {
            buffer
                .write_all(&[byte])
                .expect("Failed to write to buffer")
        });
        buffer.flush()?;
    }

    pickel.wait()?;

    Ok(())
}

/// Convert a Pixmap to a raw rgb565 image with little endian byte order.
/// This is the format used for displaying images on a Kobo.
fn to_rgb565_le(image: &Pixmap) -> impl Iterator<Item = u8> + '_ {
    image.data().chunks(4).flat_map(|chunk| {
        if let [r, g, b, alpha] = chunk {
            rgb888_to_rgb565(*r, *g, *b, *alpha)
        } else {
            // This should never happen
            panic!("Invalid image size!")
        }
    })
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
    use crate::kobo::to_rgb565_le;
    use anyhow::{Context, Result};
    use tiny_skia::Pixmap;
    use tiny_skia_path::IntSize;

    #[test]
    fn converts_all_white_image() -> Result<()> {
        let pixmap = Pixmap::from_vec(
            vec![0xFF, 0xFF, 0xFF, 0xFF],
            IntSize::from_wh(1, 1).context("Bad size")?,
        )
        .context("Bad Vec")?;

        let result = to_rgb565_le(&pixmap).collect::<Vec<u8>>();

        assert_eq!(result, vec!(0xFF, 0xFF));
        Ok(())
    }

    #[test]
    fn converts_image() -> Result<()> {
        let pixmap = Pixmap::from_vec(
            vec![0x2E, 0xD5, 0x52, 0x00],
            IntSize::from_wh(1, 1).context("Bad size")?,
        )
        .context("Bad Vec")?;

        let result = to_rgb565_le(&pixmap).collect::<Vec<u8>>();

        assert_eq!(result, vec!(0xAA, 0x2E));
        Ok(())
    }
}
