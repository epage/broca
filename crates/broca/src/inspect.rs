pub(crate) fn exec(cmd: broca_args::InspectCommand) -> Result<(), anyhow::Error> {
    match cmd {
        broca_args::InspectCommand::Discover => {
            let location = broca_config::site::SiteLocation::discover()?;
            println!("{}", serde_json::to_string(&location)?);
        }
    }

    Ok(())
}
