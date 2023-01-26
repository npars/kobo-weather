use anyhow::{Context, Result};
use log::debug;
use log::info;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;
use tiny_skia::Pixmap;

/// Kill the daemon that controls the device
pub fn kill_nickel() -> Result<()> {
    info!("Killing Nickel");
    Command::new("killall").arg("nickel").status()?;
    Ok(())
}

pub fn wifi_up() -> Result<()> {
    info!("WiFi Up");
    debug!("ifconfig eth up");
    Command::new("ifconfig").args(["eth0", "up"]).status()?;
    debug!("wlarm_le -i eth0 up");
    Command::new("wlarm_le")
        .args(["-i", "eth0", "up"])
        .status()?;
    debug!("wpa_supplicant ...");
    Command::new("wpa_supplicant")
        .args([
            "-s",
            "-i",
            "eth0",
            "-c",
            "/etc/wpa_supplicant/wpa_supplicant.conf",
            "-C",
            "/var/run/wpa_supplicant",
            "-B",
        ])
        .status()?;
    debug!("sleep...");
    sleep(Duration::from_secs(2));
    debug!("udhcpc ...");
    Command::new("udhcpc")
        .args([
            "-S",
            "-i",
            "eth0",
            "-s",
            "/etc/udhcpc.d/default.script",
            "-t15",
            "-T10",
            "-A3",
            "-f",
            "-q",
        ])
        .status()?;
    Ok(())
}

pub fn wifi_down() -> Result<()> {
    info!("WiFi Down");
    debug!("killall wpa_supplicant");
    Command::new("killall").arg("wpa_supplicant").status()?;
    debug!("wlarm_le -i eth0 down");
    Command::new("wlarm_le")
        .args(["-i", "eth0", "down"])
        .status()?;
    debug!("ifconfig eth0 down");
    Command::new("ifconfig").args(["eth0", "down"]).status()?;
    Ok(())
}

pub fn system_sleep(duration: Duration) -> Result<()> {
    info!("Putting system to sleep");
    debug!("Disabling the screen");
    {
        let mut power_state_extended = OpenOptions::new()
            .append(true)
            .open("/sys/power/state-extended")?;
        writeln!(&mut power_state_extended, "1")?;
    }

    debug!("Entering low power state");
    Command::new("/mnt/onboard/busybox_kobo")
        .args([
            "rtcwake",
            "-s",
            &duration.as_secs().to_string(),
            "-m",
            "mem",
        ])
        .status()?;

    info!("Waking up");
    debug!("Enabling the screen");
    {
        let mut power_state_extended = OpenOptions::new()
            .append(true)
            .open("/sys/power/state-extended")?;
        writeln!(&mut power_state_extended, "0")?;
    }

    Ok(())
}

pub fn display(image: Pixmap) -> Result<()> {
    info!("Displaying the dashboard");
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
