// pub struct System {}

// pub trait Info {
//     fn cameras(&self) -> Vec<Camera>;
// }

// impl Info for System {
//     #[cfg(target_os = "linux")]
//     fn cameras(&self) -> Vec<Camera> {}
//     #[cfg(target_os = "windows")]
//     fn cameras(&self) -> Vec<Camera> {}
// }

pub struct Camera {}

pub trait Snap {
    fn snap(&self) -> Vec<u8>;
}

#[cfg(target_os = "windows")]
impl Snap for Camera {
    fn snap(&self) -> Vec<u8> {
        let ctx = uvc::Context::new().expect("Could not find a context");
        let dev = ctx
            .find_device(None, None, None)
            .expect("Could not find a device");

        vec![]
    }
}

#[cfg(target_os = "linux")]
impl Snap for Camera {
    fn snap(&self) -> Vec<u8> {
        let info = rascam::info().unwrap();
        if !info.cameras.is_empty() {
            let mut camera = rascam::SimpleCamera::new(info.cameras[0].clone()).unwrap();
            camera.activate().unwrap();
            let sleep_duration = time::Duration::from_millis(1000);
            thread::sleep(sleep_duration);
            return camera.take_one().unwrap();
        }
        Vec::default()
    }
}
