use tracing::info;

mod cli;
mod api;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup and get an address to serve on
    let (addr, asset_path) = cli::init()?;

    // Server endpoints

    // Server API
    let router = axum::Router::new()
        .nest("/api", api::router())
        .route("/", axum::routing::get(index))
        .nest_service("/favicon.ico", tower_http::services::ServeFile::new(asset_path.join("favicon.ico")))
        .nest_service("/assets", tower_http::services::ServeDir::new(asset_path.clone()));

    // Listener
    info!("Serving at {addr}");
    info!("Assets at {}", asset_path.clone().display());
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Server!
    info!("Starting, hit Ctrl+C to exit");
    axum::serve(listener, router).await?;
    Ok(())
}

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl axum::response::IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

struct HtmlTemplate<T>(T);

impl<T> axum::response::IntoResponse for HtmlTemplate<T>
where
    T: askama::Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {}", err),
            )
                .into_response(),
        }
    }
}

struct BadTea(anyhow::Error);

impl axum::response::IntoResponse for BadTea {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Kablooey! {}", self.0)).into_response()
    }
}

impl<E> From<E> for BadTea
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
