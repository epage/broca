pub(crate) fn exec(cmd: broca_args::InspectCommand) -> Result<(), anyhow::Error> {
    match cmd {
        broca_args::InspectCommand::Discover => {
            let location = broca_config::site::SiteLocation::discover()?;
            println!("{}", serde_json::to_string(&location)?);
        }
        broca_args::InspectCommand::Config => {
            let site = broca_config::site::Site::load()?;
            println!("{}", serde_json::to_string(&site)?);
        }
    }

    Ok(())
}
