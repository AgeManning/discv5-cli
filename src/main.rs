#![doc=include_str!("../README.md")]





#[tokio::main]
async fn main() {
    let cli_matches = cli::start_cli();

    // Set up the logging
    let log_level = match cli_matches
        .value_of("log-level")
        .expect("Log level must be present")
    {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => unreachable!(),
    };

    // Initialize the logger
    if simple_logger::SimpleLogger::new()
        .with_level(log_level)
        .with_utc_timestamps()
        .init()
        .is_err()
    {
        log::error!("Failed to initialize logger. Please try again.");
    }

    // Parse the CLI parameters.
    if let Some(server_matches) = cli_matches.subcommand_matches("server") {
        server::run(server_matches).await;
    } else if let Some(enr_matches) = cli_matches.subcommand_matches("request-enr") {
        request_enr::run(enr_matches).await;
    } else if let Some(packet_matches) = cli_matches.subcommand_matches("packet") {
        if let Some(decode_matches) = packet_matches.subcommand_matches("decode") {
            packet::decode(decode_matches);
        } else {
            log::error!("A packet subcommand must be supplied. See --help for options");
        }
    } else {
        log::error!("A subcommand must be supplied. See --help for options");
    }
}
