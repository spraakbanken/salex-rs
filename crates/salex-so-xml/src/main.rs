use clap::{value_parser, Arg, Command};
use salex_so_infrastructure::connect;
use salex_so_infrastructure::telemetry;
// use sqlx::mysql::MySqlPoolOptions;
use std::error::Error;
use std::path::PathBuf;
use std::process;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let subscriber = telemetry::get_subscriber("salex-so".into(), "trace".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    // env_logger::Builder::from_env(Env::default().default_filter_or("trace"))
    //     .format_timestamp(None)
    //     .init();
    tracing::trace!("Hello, world!");
    if let Err(err) = try_main().await {
        tracing::error!("Error: {:?}", err);
        process::exit(1);
    }
}

async fn try_main() -> Result<(), Box<dyn Error>> {
    tracing::trace!("try_main");
    let db_url = std::env::var("DATABASE_URL")?;
    let matches = cli().get_matches();

    // let configuration = get_configuration().expect("Failed to read configuration");

    let pool = connect(&db_url).await?;
    match matches.subcommand() {
        Some(("so-underlag", sub_matches)) => {
            let output = sub_matches.get_one::<PathBuf>("output").unwrap();
            tracing::info!("Writing so-underlag to {}", output.display());
        }
        _ => unreachable!(),
    }
    Ok(())
}

fn cli() -> Command {
    Command::new("salex-so")
        .about("Export SALEX to SO")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("so-underlag").arg(
                Arg::new("output")
                    .short('o')
                    .long("output")
                    .num_args(1)
                    .value_name("FILE")
                    .help("Path to write to")
                    .required(true)
                    .value_parser(value_parser!(PathBuf)),
            ),
        )
}
