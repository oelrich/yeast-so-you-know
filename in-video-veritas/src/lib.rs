use tracing::{event, Level};
use std::{thread, time};

pub fn get_the_picture() -> Vec<u8> {
    let info = rascam::info().unwrap();
    if !info.cameras.is_empty() {
        let mut camera = rascam::SimpleCamera::new(info.cameras[0].clone()).unwrap();
        camera.activate().unwrap();
        let sleep_duration = time::Duration::from_millis(2000);
        thread::sleep(sleep_duration);
        return camera.take_one().unwrap();
    }
    Vec::default()    
}

// pub fn get_the_picture() -> Result<(), &'static str> {
//     match rascam::info() {
//         Ok(info) => {
//             if info.cameras.len() > 0 {
//                 match info.cameras[0].activate() {
//                     Ok()
//                 }
//             } else {
//                 Err("no camera available")
//             }
//         }
//         Err(error) => {
//             event!(target: "in-video-veritas", Level::ERROR,"{}", error);
//             Err("could not get camera")
//         }
//     }
// }

pub fn have_camera() -> bool {
    match rascam::info() {
        Ok(info) => !info.cameras.is_empty(),
        Err(_error) => false,
    }
}

pub fn report_cameras() {
    match rascam::info() {
        Ok(info) => {
            for camera in info.cameras {
                //let camera = format!("{}", camera);
                event!(target: "in-video-veritas", Level::INFO,"{}", camera);
            }
        }
        Err(err) => println!("{}", err),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_camera() {
        assert!(super::have_camera());
    }
}
