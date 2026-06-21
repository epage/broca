use clap::Parser;

mod inspect;

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{err}");
        }
    }
}

fn run() -> Result<(), anyhow::Error> {
    let args = broca_args::Cli::parse();

    match args.command {
        broca_args::CliCommand::Inspect(cmd) => inspect::exec(cmd),
    }
}
