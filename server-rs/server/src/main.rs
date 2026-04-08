use clap::Parser;
use config::{
    AppConfig, DEFAULT_DB_URL, DEFAULT_FRONT_END_HOST, DEFAULT_PORT, ENV_DATABASE_URL,
    ENV_FRONT_END_HOST, ENV_PORT,
};
use database::connect;
use mplat_server::cli::{Cli, Commands, install_systemd_service, update};
use mplat_server::server::start_server;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    dotenv::dotenv().ok();

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let port = std::env::var(ENV_PORT)
        .ok()
        .map_or(Ok(DEFAULT_PORT), |env_val| env_val.parse::<u16>())?;

    let database_url = std::env::var(ENV_DATABASE_URL).unwrap_or(DEFAULT_DB_URL.to_string());

    let front_end_host =
        std::env::var(ENV_FRONT_END_HOST).unwrap_or(DEFAULT_FRONT_END_HOST.to_string());

    let db_pool = connect(&database_url).await?;
    let config = AppConfig::load(&db_pool).await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Run => {
            start_server(config, db_pool, port, front_end_host).await?;
        }
        Commands::Install {
            bin,
            log_file,
            user,
            group,
        } => {
            install_systemd_service(&bin, &log_file, user.as_deref(), group.as_deref())?;
        }
        Commands::Version => {
            println!("mplat server v{}", env!("CARGO_PKG_VERSION"));
        }
        Commands::Update { bin } => {
            update(&bin)?;
        }
    }

    Ok(())
}
