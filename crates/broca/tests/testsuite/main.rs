mod ui;

fn broca() -> snapbox::cmd::Command {
    snapbox::cmd::Command::cargo_bin("broca").with_assert(assert())
}

fn assert() -> snapbox::Assert {
    let mut redactions = snapbox::Redactions::new();
    redactions
        .insert("[EXE]", std::env::consts::EXE_SUFFIX)
        .unwrap();
    redactions
        .insert(
            "[CARGO_MANIFEST_DIR]",
            std::env::current_dir()
                .unwrap()
                .into_os_string()
                .into_string()
                .unwrap()
                .replace(std::path::MAIN_SEPARATOR_STR, "/"),
        )
        .unwrap();

    snapbox::Assert::new()
        .action_env(snapbox::assert::DEFAULT_ACTION_ENV)
        .redact_with(redactions)
}
