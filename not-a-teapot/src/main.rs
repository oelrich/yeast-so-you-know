use warp::{http::Response, Filter};

mod cli;

#[tokio::main]
async fn main() {
    let addr = cli::init();
    let hello = warp::path!("hello" / String).map(|name| format!("Hello there, {}!", name));
    let camera = warp::path!("camera").map(|| {
        Response::builder()
            .header("content-type", "image/jpeg")
            .body(in_video_veritas::get_the_picture())
    });
    let api = warp::any().and(hello.or(camera));
    warp::serve(api).run(addr).await;
}
