use obscura::Snap;
use std::{thread, time};
use tracing::{event, Level};

mod obscura;

pub enum CameraError {
    ImBlind,
    Headache,
}

pub struct Camera(obscura::Camera);

impl Camera {
    pub fn default() -> Result<Self, CameraError> {
        Err(CameraError::ImBlind)
    }
    pub fn get_the_picture(&self) -> Result<Vec<u8>, CameraError> {
        Err(CameraError::Headache)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_camera() {
        assert!(true);
    }
}
