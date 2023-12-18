
use tracing::{event, Level};

pub fn get_the_picture() -> anyhow::Result<Vec<u8>> {
    event!(Level::DEBUG, "snap");
    let index = nokhwa::utils::CameraIndex::Index(0);
    let requested = nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::RgbFormat>(nokhwa::utils::RequestedFormatType::AbsoluteHighestResolution);

    let mut camera = nokhwa::Camera::new(index, requested)?;
    let frame = camera.frame()?;

    let image = frame.decode_image::<nokhwa::pixel_format::RgbFormat>()?;
    
    let mut bytes: Vec<u8> = Vec::new();

    image.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

    Ok(bytes)
}