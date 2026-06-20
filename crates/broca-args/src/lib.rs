//! > DESCRIPTION

#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(clippy::print_stderr)]
#![warn(clippy::print_stdout)]

#[derive(clap::Parser)]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub struct Cli {}
