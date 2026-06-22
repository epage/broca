use clap::Parser;

mod inspect;

fn main() {
    setup_logger();

    match run() {
        Ok(()) => {}
        Err(err) => {
            anstream::eprintln!("{err}");
        }
    }
}

fn run() -> Result<(), anyhow::Error> {
    let args = broca_args::Cli::parse();

    match args.command {
        broca_args::CliCommand::Inspect(cmd) => inspect::exec(cmd),
    }
}

fn setup_logger() -> Option<tracing_chrome::FlushGuard> {
    use tracing_subscriber::prelude::*;

    let env = tracing_subscriber::EnvFilter::from_env("BROCA_LOG");
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(tracing_subscriber::fmt::time::Uptime::default())
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .with_filter(env);

    let (profile_layer, profile_guard) = chrome_layer();

    let registry = tracing_subscriber::registry()
        .with(fmt_layer)
        .with(profile_layer);
    registry.init();
    tracing::trace!(start = jiff::Timestamp::now().to_string());
    profile_guard
}

fn chrome_layer<S>() -> (
    Option<tracing_chrome::ChromeLayer<S>>,
    Option<tracing_chrome::FlushGuard>,
)
where
    S: tracing::Subscriber
        + for<'span> tracing_subscriber::registry::LookupSpan<'span>
        + Send
        + Sync,
{
    if env_to_bool(std::env::var_os("BROCA_LOG_PROFILE").as_deref()) {
        let capture_args =
            env_to_bool(std::env::var_os("BROCA_LOG_PROFILE_CAPTURE_ARGS").as_deref());
        let (layer, guard) = tracing_chrome::ChromeLayerBuilder::new()
            .include_args(capture_args)
            .build();
        (Some(layer), Some(guard))
    } else {
        (None, None)
    }
}

fn env_to_bool(os: Option<&std::ffi::OsStr>) -> bool {
    matches!(os.and_then(|os| os.to_str()), Some("1") | Some("true"))
}
