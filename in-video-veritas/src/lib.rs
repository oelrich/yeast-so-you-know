
use tracing::{debug, info};

#[cfg(target_os = "windows")]
pub fn get_the_picture() -> anyhow::Result<Vec<u8>> {
    info!("snap");
    let index = nokhwa::utils::CameraIndex::Index(0);
    let requested = nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(nokhwa::utils::RequestedFormatType::None);
    let mut camera = nokhwa::Camera::new(index, requested)?;
    let frame = camera.frame()?;
    let image = frame.decode_image::<nokhwa::pixel_format::RgbFormat>()?;
    let mut bytes: Vec<u8> = Vec::new();
    image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;
    Ok(bytes)
}

#[cfg(target_os = "linux")]
pub fn get_the_picture() -> anyhow::Result<Vec<u8>> {
    let info = rascam::info()?;
    info!("snap");
    debug!("{:?}", info.cameras);
    if !info.cameras.is_empty() {
            let mut camera = rascam::SimpleCamera::new(info.cameras[0].clone())?;
            camera.activate()?;
            let sleep_duration = std::time::Duration::from_millis(1000);
            std::thread::sleep(sleep_duration);
            let picture = camera.take_one()?;
            Ok(picture)
    } else {
        Err(anyhow::anyhow!("No camera found"))
    }
}
