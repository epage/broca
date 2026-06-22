pub(crate) fn exec(cmd: broca_args::InspectCommand) -> Result<(), anyhow::Error> {
    match cmd {
        broca_args::InspectCommand::Root => {
            let location = broca_config::site::Root::discover()?;
            anstream::println!("{}", serde_json::to_string(&location)?);
        }
        broca_args::InspectCommand::Config => {
            let site = broca_config::site::Site::load()?;
            anstream::println!("{}", serde_json::to_string(&site)?);
            if !site._unused.is_empty() {
                for field in &site._unused {
                    let mut group = annotate_snippets::Group::with_title(
                        annotate_snippets::Level::WARNING
                            .primary_title(format!("unused field `{field}`")),
                    );
                    if let Some(broca_config::site::Root::Config(config)) = &site._root {
                        group = group.element(annotate_snippets::Origin::path(config.as_str()));
                    }
                    let report = &[group];
                    let renderer = annotate_snippets::renderer::Renderer::styled();
                    let report = renderer.render(report);
                    anstream::eprintln!("{report}");
                }
            }
        }
    }

    Ok(())
}
