async fn camera() -> Result<impl axum::response::IntoResponse, crate::BadTea> {
    let image = in_video_veritas::get_the_picture()?;
    Ok((
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        image,
    ))
}

fn camera_router() -> axum::Router {
    axum::Router::new().route("/snap", axum::routing::get(camera))
}

pub fn router() -> axum::Router {
    axum::Router::new().nest("/camera", camera_router())
}