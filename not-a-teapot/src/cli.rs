use clap::Parser;

use tracing::{warn, info, error};

fn get_address(addr: &Option<std::net::SocketAddr>) -> std::net::SocketAddr {
    if let Some(address) = addr {
        return *address;
    }
    if let Some(address) = get_env_addr("YEAST_ADDR") {
        return address;
    }
    std::net::SocketAddr::from(([0, 0, 0, 0], 80))
}

fn get_log_filter(logging: &Option<String>) -> String {
    if let Some(log_filter) = logging {
        return log_filter.to_owned();
    }
    if let Ok(log_filter) = std::env::var("YEAST_LOG_LEVEL") {
        return log_filter;
    }
    String::from("INFO")
}

pub fn init() -> anyhow::Result<(std::net::SocketAddr, std::path::PathBuf)> {
    use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};
    let cli = Cli::parse();

    let log_filter = get_log_filter(&cli.log_filter);

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting");
    Ok((get_address(&cli.address), get_asset_root(cli.serve_dir)?))
}

#[derive(Parser, Debug)]
#[structopt(about = "Use `-h` for help!", rename_all = "kebab-case")]
struct Cli {
    #[structopt(help = "Where should I serve stuff?", long, short)]
    address: Option<std::net::SocketAddr>,
    #[structopt(help = "What should I log?", long, short)]
    log_filter: Option<String>,
    #[structopt(help = "What directory should I serve from?", long, short)]
    serve_dir: Option<std::path::PathBuf>
}

fn get_env_addr(name: &str) -> Option<std::net::SocketAddr> {
    // This is *not* pretty ...
    std::env::var(name)
        .ok()
        .map(|addr| match addr.parse() {
            Ok(address) => Some(address),
            Err(_err) => {
                error!("{} provided but not an address", name);
                None
            }
        })
        .flatten()
}

fn get_asset_root(cli_path : Option<std::path::PathBuf>) -> anyhow::Result<std::path::PathBuf> {
    if let Some(path) = cli_path {
        Ok(path)
    } else {
        let current_path = std::env::current_dir()?;
        warn!("Asset path not specified, guessing from {}", current_path.clone().display());
        let asset_path = current_path.join("assets");
        if asset_path.exists() {
            Ok(asset_path)
        } else {
            let asset_path = current_path.join("not-a-teapot").join("assets");
            if asset_path.exists() {
                Ok(asset_path)
            } else {
                Err(anyhow::anyhow!("Found no assets from {}", current_path.display()))
            }
        }
    }
}
