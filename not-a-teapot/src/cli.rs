use structopt::StructOpt;
fn get_address(addr: &Option<std::net::SocketAddr>) -> std::net::SocketAddr {
    if let Some(address) = addr {
        return *address;
    }
    if let Some(address) = get_env_addr("YEAST_ADDR") {
        return address;
    }
    std::net::SocketAddr::from(([0, 0, 0, 0], 8080))
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

pub fn init() -> std::net::SocketAddr {
    use tracing_subscriber::{fmt::format::FmtSpan, FmtSubscriber};
    let cli = Cli::from_args();

    let log_filter = get_log_filter(&cli.log_filter);

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    get_address(&cli.address)
}

#[derive(StructOpt)]
#[structopt(about = "Use `-h` for help!", rename_all = "kebab-case")]
struct Cli {
    #[structopt(help = "Where should i serve stuff?", long, short)]
    address: Option<std::net::SocketAddr>,
    #[structopt(help = "What should I log?", long, short)]
    log_filter: Option<String>,
}

fn get_env_addr(name: &str) -> Option<std::net::SocketAddr> {
  // This is *not* pretty ...
  std::env::var(name).ok().map(|addr|addr.parse().ok()).flatten()
}
