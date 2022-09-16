use warp::{http::Response, Filter};

mod cli;

#[derive(Debug)]
enum Action {
    Up,
    Down,
}

impl std::fmt::Display for Action {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Up => write!(fmt, "Go UP!"),
            Self::Down => write!(fmt, "Go DOWN!"),
        }
    }
}

impl std::str::FromStr for Action {
    type Err = u128;
    fn from_str(input: &str) -> std::result::Result<Self, <Self as std::str::FromStr>::Err> {
        match input {
            "up" => Ok(Self::Up),
            "down" => Ok(Self::Down),
            _ => Err(404),
        }
    }
}

#[tokio::main]
async fn main() {
    // Setup and get an address to serve on
    let addr = cli::init();
    // Server endpoints
    let hello = warp::path!("hello" / String).map(|name| format!("Hello there, {}!", name));
    let good_bye = warp::path!("bye").map(|| "Bye!");

    let next_value = warp::path!("do" / Action).map(|action| format!("Action: {}", action));

    let camera = warp::path!("camera").map(|| {
        Response::builder()
            .header("content-type", "image/jpeg")
            .body(in_video_veritas::get_the_picture())
    });

    // Server API
    let api = warp::any().and(hello.or(good_bye).or(next_value).or(camera));

    // Server!
    warp::serve(api).run(addr).await;
}
