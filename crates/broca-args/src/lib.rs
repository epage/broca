//! > DESCRIPTION

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[derive(Debug, clap::Parser)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum CliCommand {
    #[command(subcommand)]
    Inspect(InspectCommand),
}

#[derive(Debug, clap::Subcommand)]
pub enum InspectCommand {
    Root,
    Config,
}
